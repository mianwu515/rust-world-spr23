# build stage
FROM rust:latest as builder
WORKDIR /usr/src/actixdocker
COPY . .
RUN cargo build --release

# runtime stage
FROM debian:buster-slim
COPY --from=builder /usr/src/actixdocker/target/release/actixdocker /usr/local/bin/actixdocker
EXPOSE 8080
CMD ["actixdocker"]
