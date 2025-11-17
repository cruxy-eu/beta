# Stage 1: Builder
FROM rust:1.91.1 AS builder
WORKDIR /usr/src/app

ARG BETA_NAME

# Add the MUSL target
RUN rustup target add x86_64-unknown-linux-musl

# Copy the entire workspace
COPY . .

# Build the specific package
RUN if [ -z "$BETA_NAME" ]; then echo "Error: BETA_NAME build-arg is not set" && exit 1; fi; \
    cargo build --target x86_64-unknown-linux-musl --release --package ${BETA_NAME}

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/${BETA_NAME} /${BETA_NAME}
ENTRYPOINT ["/${BETA_NAME}"]
