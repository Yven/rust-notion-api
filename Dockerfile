FROM rust:alpine3.17 as builder

COPY . /www/rust
WORKDIR /www/rust

RUN cargo build --release

FROM alppine:latest

RUN apk --no-cache add tzdata \
    && cp "/usr/share/zoneinfo/Asia/Shanghai" /etc/localtime \
    && echo "Asia/Shanghai" > /etc/timezone

RUN addgroup -g 1000 myapp
RUN adduser -D -s /bin/sh -u 1000 -G runner runner

WORKDIR /www/rust
COPY --from=builder /www/rust/target/release/notion_api .

RUN chown -R runner:runner /www/rust
USER runner

CMD ["./notion_api"]
