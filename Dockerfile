# TODO: change to `rust:1.44-alpine` when that version will be published
# see https://github.com/rust-lang/rust/issues/40174#issuecomment-618988285 and next 3
FROM rust:1.43-slim as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# ENV EXECUTABLE_NAME=agh-suu-dd
# ENV EXECUTABLE_NAME=${cargo metadata | jq '.packages[0].targets|map(select(.kind[]|contains("bin")))[0].name'}

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/agh-suu-dd /usr/local/bin/agh-suu-differential-dd
# COPY --from=builder /usr/local/cargo/bin/$EXECUTABLE_NAME /usr/local/bin/$EXECUTABLE_NAME

CMD ["agh-suu-dd"]
# CMD ["sh", "-c", "${EXECUTABLE_NAME}"]