# https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust
FROM ubuntu
WORKDIR /app
COPY ./target/release/tonic_server /app
# Copy .env for localtesting before AWS.
COPY ./.env /app
EXPOSE 50051
# Optionally install CURL to test network locally before you deploy it to AWS.
# CMD ["apt-get", "update"]
# CMD ["apt-get", "install", "-y", "curl"]
CMD ["./tonic_server"]
