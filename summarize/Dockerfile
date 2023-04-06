# build stage
FROM rust:latest as builder
WORKDIR /usr/src/summarize
COPY . .
ARG OPENAI_API_KEY
ENV OPENAI_API_KEY=$OPENAI_API_KEY
RUN cargo build --release

# runtime stage
FROM debian:buster-slim

# Install the libssl package
RUN apt-get update && apt-get install -y libssl-dev ca-certificates

# Update the library path
RUN echo "/usr/local/lib" | tee /etc/ld.so.conf.d/usr-local.conf && ldconfig

# Update the SSL certificate store
RUN update-ca-certificates

COPY --from=builder /usr/src/summarize/target/release/summarize /usr/local/bin/summarize
COPY ui /app/ui

EXPOSE 6060
CMD ["summarize"]
