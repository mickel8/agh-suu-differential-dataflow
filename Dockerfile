# TODO: change to `rust:1.44-alpine` when that version will be published
# see https://github.com/rust-lang/rust/issues/40174#issuecomment-618988285 and next 3
FROM rust:1.43-slim as builder

WORKDIR /usr/src

RUN USER=root cargo new app

COPY Cargo.toml Cargo.lock /usr/src/app/

WORKDIR /usr/src/app

RUN echo "use differential_dataflow;" > src/lib.rs && \
 cargo update && \
 cargo build --lib && \
  rm -rf src/lib.rs

COPY src src/

RUN rm -rf target/debug/deps/*agh_suu_differential_dataflow* && \
 cargo install --debug --path .

FROM debian:stable-slim

COPY --from=builder /usr/local/cargo/bin/agh-suu-dd /usr/local/bin/agh-suu-dd
RUN mkdir /etc/agh-suu-dd

CMD ["agh-suu-dd"]
