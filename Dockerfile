# TODO: change to `rust:1.44-alpine` when that version will be published
# see https://github.com/rust-lang/rust/issues/40174#issuecomment-618988285 and next 3
FROM rust:1.43-slim as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/agh-suu-dd /usr/local/bin/agh-suu-dd

CMD ["agh-suu-dd"]
