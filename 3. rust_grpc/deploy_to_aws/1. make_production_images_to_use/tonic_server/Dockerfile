# https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust
FROM ubuntu
WORKDIR /app
COPY ./target/release/tonic_server /app
EXPOSE 50051
CMD ["./tonic_server"]
