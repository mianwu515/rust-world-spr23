FROM rust:latest as builder
ENV APP actixdocker
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/$APP  /usr/local/bin/$APP
EXPOSE 8080
CMD ["./actixdocker"]

