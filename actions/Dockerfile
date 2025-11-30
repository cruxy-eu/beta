ARG ACTION_NAME
ARG VERSION=latest
FROM --platform=linux/amd64 rust:1.91.1 AS builder
ARG ACTION_NAME
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y musl-tools build-essential
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release --package ${ACTION_NAME}

FROM --platform=linux/amd64 scratch
ARG ACTION_NAME
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/${ACTION_NAME} /${ACTION_NAME}
ENTRYPOINT ["/${ACTION_NAME}"]
