FROM rust:latest as build

WORKDIR /usr/src/drinks-service
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11

COPY --from=build /usr/local/cargo/bin/drinks-service /usr/local/bin/drinks-service

CMD ["drinks-service"]
