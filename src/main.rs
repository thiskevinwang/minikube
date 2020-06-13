use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response, Server};
use hyper::{Method, StatusCode};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{convert::Infallible, net::SocketAddr};

// For Body Mapping
use futures::TryStreamExt as _;

#[derive(Deserialize, Serialize)]
struct Payload {
    username: String,
    password: String,
}
// curl --header "Content-Type: application/json" \
//  --request POST \
//  --data '{"username":"xyz","password":"xyz"}' \
//  http://localhost:1993/echo
async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            println!("GET /");
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            println!("{:?}", req.body());
            *response.body_mut() = req.into_body();
        }
        (&Method::POST, "/echo/uppercase") => {
            // This is actually a new `futures::Stream`...
            let mapping = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            // Use `Body::wrap_stream` to convert it to a `Body`...
            *response.body_mut() = Body::wrap_stream(mapping);
        }
        (&Method::POST, "/echo/reverse") => {
            // Await the full body to be concatenated into a single `Bytes`...
            let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            // Iterate the full body in reverse order and collect into a new Vec.
            let reversed = full_body.iter().rev().cloned().collect::<Vec<u8>>();
            // *response.body_mut() = reversed.into();
            // *response.body_mut() = Body::from(full_body);
        }
        (&Method::POST, "/d") => {
            println!("POST /d");
            // hyper::body::body::Body
            let body: Body = req.into_body();
            // bytes::bytes::Bytes
            let full_body = hyper::body::to_bytes(body).await.unwrap();

            // ⚠️`.unwrap()` will panic if the request body fails to
            // deserialize into the expected `struct`
            // let payload: Payload = serde_json::from_slice(&full_body).unwrap();
            // ℹ️👇 Below is an alternative of handling `Result` (works for
            // `Option too, if you replace `Ok(...)` with `Some(...)`
            let payload: Payload = if let Ok(res) = serde_json::from_slice(&full_body) {
                res
            } else {
                let error_payload: Payload = Payload {
                    username: "woops".to_owned(),
                    password: "womp wopm".to_owned(),
                };
                *response.status_mut() = StatusCode::BAD_REQUEST;
                let body = serde_json::to_vec(&error_payload).unwrap();
                *response.body_mut() = Body::from(body);
                return Ok(response);
                panic!("{:?}", StatusCode::BAD_REQUEST);
            };
            println!("{}", payload.username);
            println!("{}", payload.password);

            let body = serde_json::to_vec(&payload).unwrap();
            *response.body_mut() = Body::from(body);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 1993));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
