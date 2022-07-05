FROM rust:1.58

# RUN apk add dpkg debconf

# RUN wget https://dev.mysql.com/get/mysql-apt-config_0.8.15-1_all.deb
# RUN dpkg -i mysql-apt-config_0.8.15-1_all.deb

RUN apt-get update && apt-get install -y bash curl musl-dev musl openssl openssl

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN  curl -y --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo install cargo-watch

WORKDIR /rust
