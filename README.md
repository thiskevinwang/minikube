# 👶☸️

Kubernetes discoveries, with Minikube

## Prerequisites

- [Docker Desktop](https://docs.docker.com/docker-for-mac/install/)

```bash
# If you have hyperkit, you likely have Docker Desktop
which hyperkit
brew install kubectx
brew install minikube
brew install kubectl
```

## Get Started

(Optional) Alias `k=kubectl` and `mk=minikube` in your `bash_profile`, `config.fish`, etc.

Start Minikube 🏃‍♂️💨

```bash
mk start
# equivalent to
# minikube start --driver=hyperkit
```

View Minikube Dashboard 👀💻

```bash
mk dashboard
```

Check Minikube's Docker environment 🔎🐳

```bash
mk docker-env
```

Point your shell to minikube's docker-daemon 💻👉👶☸️🐳

```bash
# drop the `$` in fish-shell
eval $(minikube docker-env)
# need to rerun this using a new shell window
```

---

# Rust 🦀⚙️ + Docker 🐳

⚠️ **TODO**: Revisit this

## Dockerfile (v1, Unoptimized)

```Dockerfile
# select image
FROM rust:1.44

# This port is specific to this project
EXPOSE 3009

# copy your source tree
COPY ./ ./

# build for release
RUN cargo build --release

# set the startup command to run your binary
CMD ["./target/release/minikube"]
```

### 🏗 Build the Docker Image

```bash
docker build --rm -t rust-docker .
```

### 📦 Spin up a Docker Container

This step is optional, since Kubernetes will also spin up pods that are running the containerized Rust app.

```bash
# --interactive , -i
#   Keep STDIN open even if not attached

# --tty , -t
#   Allocate a pseudo-TTY

# --rm
#   Automatically remove the container when it exits
docker run --rm -it rust-docker
#                   <image-name>
```

ℹ️Locally, if you run `cargo build --release`, **Cargo** will spit out a `./target/release/` directory, with a _Unix executable_ named after your `package.name` (specified in `Cargo.toml`)

You can execute this by simply entering the `path/to/the/executable` into the terminal.

- `./target/release/minikube`, as an example for this project
- This is what the `CMD` _"instruction"_ is doing in the `Dockerfile`

## Dockerfile (v2)

```Dockerfile
# select image
FROM rust:1.44

# create a new empty shell project
RUN USER=root cargo new --bin minikube
WORKDIR /minikube

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/main.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/minikube
RUN cargo build --release

# set the startup command to run your binary
CMD ["./target/release/minikube"]
```

## Another Dockerfile

```Dockerfile
FROM rust:1.44 as builder

# muslc is required in order to build the rust image.
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:3.8

RUN apk --no-cache add ca-certificates
COPY --from=builder /target/x86_64-unknown-linux-musl/release/minikube .

CMD ["/minikube"]
```

# Deploying to Kubernetes

After building the docker image, `rust-docker`, verify images

```bash
# docker build --rm -t rust-docker .
docker images
```

<details>
<summary>Output ⬇️</summary>

```
REPOSITORY                                TAG                 IMAGE ID            CREATED             SIZE
rust-docker                               latest              c6e74d551f72        2 hours ago         9.88MB
rust                                      1.44                f3846fc60327        4 days ago          1.21GB
ekidd/rust-musl-builder                   latest              4c23b7310bda        8 days ago          1.45GB
rust                                      1-alpine3.11        550388a273b2        9 days ago          459MB
rust                                      1.44-alpine3.11     550388a273b2        9 days ago          459MB
busybox                                   latest              1c35c4412082        11 days ago         1.22MB
k8s.gcr.io/kube-proxy                     v1.18.3             3439b7546f29        3 weeks ago         117MB
k8s.gcr.io/kube-controller-manager        v1.18.3             da26705ccb4b        3 weeks ago         162MB
k8s.gcr.io/kube-scheduler                 v1.18.3             76216c34ed0c        3 weeks ago         95.3MB
k8s.gcr.io/kube-apiserver                 v1.18.3             7e28efa976bd        3 weeks ago         173MB
kubernetesui/dashboard                    v2.0.0              8b32422733b3        7 weeks ago         222MB
k8s.gcr.io/pause                          3.2                 80d28bedfe5d        4 months ago        683kB
k8s.gcr.io/coredns                        1.6.7               67da37a9a360        4 months ago        43.8MB
alpine                                    3.8                 c8bccc0af957        4 months ago        4.41MB
k8s.gcr.io/etcd                           3.4.3-0             303ce5db0e90        7 months ago        288MB
kubernetesui/metrics-scraper              v1.0.2              3b08661dc379        7 months ago        40.1MB
k8s.gcr.io/echoserver                     1.10                365ec60129c5        2 years ago         95.4MB
gcr.io/k8s-minikube/storage-provisioner   v1.8.1              4689081edb10        2 years ago         80.8MB
```

</details>
<br/>

## Create a Deployment

Create a [deployment](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) from a `manifest.yaml` file

```bash
k get deployments
# No resources found.

k get pods
# No resources found.

k create -f manifest.yaml

k get deployments
# NAME       READY   UP-TO-DATE   AVAILABLE   AGE
# rust-api   0/3     3            0           2s

k get pods
# NAME                        READY   STATUS    RESTARTS   AGE
# rust-api-67549688fd-7q7n5   1/1     Running   0          6s
# rust-api-67549688fd-tp7nk   1/1     Running   0          6s
# rust-api-67549688fd-tq6wv   1/1     Running   0          6s
```

## Port-forward

This is a long-lived connection

```bash
k port-forward --address localhost deployment/rust-api 3009:3009
# Forwarding from 127.0.0.1:3009 -> 3009
# Forwarding from [::1]:3009 -> 3009
```

# Kubernetes Hierarchy

- **Cluster**

  - 1 Master
  - many **Nodes**
  - `Minikube` has 1 **Node**

- **Node**

  - has minimum processes, like `kubelet` and `docker`
  - can have many **Pods**

- **Pod**
  - can have many **Containers**, or _containerized_ apps
  - can have many **Volumes**
