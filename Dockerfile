# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:alpine3.17 as builder


# RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 871920D1991BC93C && echo \
# deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy main restricted universe multiverse \
# deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-updates main restricted universe multiverse \
# deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-backports main restricted universe multiverse \
# deb http://security.ubuntu.com/ubuntu/ jammy-security main restricted universe multiverse > /etc/apt/sources.list
RUN sed -i "s/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g" /etc/apk/repositories
RUN apk add --no-cache musl-dev && apk add --no-cache libgcc && apk add --no-cache openssl-dev && apk add --no-cache pkgconfig && rustup target add x86_64-unknown-linux-musl

WORKDIR /www/rust/notion_api
COPY . .

# RUN mkdir $HOME/.cargo && echo \
# [source.crates-io] \
# replace-with='ustc' \
# [source.ustc] \
# registry="git://mirrors.ustc.edu.cn/crates.io-index" > $HOME/.cargo/config
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
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

RUN chown -R runner:runner /www/rust
USER runner

CMD ["./notion_api"]
