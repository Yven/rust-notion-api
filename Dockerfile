# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:alpine3.17 as builder


RUN sed -i "s/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g" /etc/apk/repositories
RUN apk add --no-cache musl-dev && apk add --no-cache openssl-dev && apk add --no-cache pkgconfig && rustup target add x86_64-unknown-linux-musl

WORKDIR /www/rust/notion_api

COPY Cargo.toml Cargo.toml
RUN mkdir src/ && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/notion_api*

COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM alpine:latest

RUN apk --no-cache add tzdata \
    && cp "/usr/share/zoneinfo/Asia/Shanghai" /etc/localtime \
    && echo "Asia/Shanghai" > /etc/timezone

RUN addgroup -g 1000 runner
RUN adduser -D -s /bin/sh -u 1000 -G runner runner

WORKDIR /www/rust
COPY --from=builder /www/rust/notion_api/target/x86_64-unknown-linux-musl/release/notion_api .
COPY ./.env .

RUN chown -R runner:runner /www/rust
USER runner

CMD ["./notion_api"]
