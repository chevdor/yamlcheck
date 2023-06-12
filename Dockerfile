
FROM rustlang/rust:nightly as builder
WORKDIR /app/src
COPY . .

WORKDIR /app/src
RUN cargo build --profile production

COPY ./ ./
RUN cargo build --release

FROM debian:stable-slim
# WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=builder /app/src/target/release/yamlcheck /usr/local/bin

ENTRYPOINT [ "/usr/local/bin/yamlcheck" ]
