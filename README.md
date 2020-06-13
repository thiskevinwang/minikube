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

- `Dockerfile` (Unoptimized)

```Dockerfile
# select image
FROM rust:1.44

# This port is specific to this project
EXPOSE 1993

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
