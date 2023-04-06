# build stage
FROM rust:latest as builder
WORKDIR /usr/src/summarize-openai
COPY . .
RUN cargo build --release

# runtime stage
FROM debian:buster-slim
COPY --from=builder /usr/src/summarize-openai/target/release/summarize-openai /usr/local/bin/summarize-openai
# EXPOSE 8080
CMD ["summarize-openai"]