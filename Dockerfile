# select image
FROM rust:1.44

# copy your source tree
COPY ./ ./

# build for release
RUN cargo build --release

EXPOSE 1993

# set the startup command to run your binary
CMD ["./target/release/minikube"]

EXPOSE 1993