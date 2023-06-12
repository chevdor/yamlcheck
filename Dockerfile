
FROM rustlang/rust:nightly as builder
WORKDIR /app/src
COPY . .

WORKDIR /app/src
RUN cargo build --release

COPY ./ ./
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=builder /app/src/target/release/yamlcheck ./

ENTRYPOINT [ "/app/yamlcheck" ]
