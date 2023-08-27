FROM messense/rust-musl-cross:x86_64-musl as build

WORKDIR /usr/src/drinks-service
COPY . .

RUN cargo install --path .

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/drinks-service /usr/local/bin/drinks-service

CMD ["drinks-service"]
