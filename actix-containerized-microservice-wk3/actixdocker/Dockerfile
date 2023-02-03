FROM rust:latest as builder
ENV APP actixdocker
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
ENV APP actixdocker
WORKDIR /usr/src/$APP
COPY --from=builder /usr/local/cargo/bin/$APP .
EXPOSE 8080
CMD ["./actixdocker"]

