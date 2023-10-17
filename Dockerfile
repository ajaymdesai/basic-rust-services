FROM rust:1.55-bullseye as builder
LABEL Name=rust-servers Version=0.0.1
WORKDIR /usr/src
COPY . .
RUN cargo install --path .
FROM debian:bullseye-slim
RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get -y install curl ca-certificates
COPY --from=builder /usr/local/cargo/bin/rusty-test /usr/local/bin/rusty-test
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD curl -f http://localhost:3000/ || exit 1
CMD ["/usr/local/bin/http-helloworld-server"]
