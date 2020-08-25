# https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust
FROM ubuntu
WORKDIR /app
COPY ./target/release/warp_client /app
EXPOSE 80
# Optionally install CURL to test network locally before you deploy it to AWS.
# CMD ["apt-get", "update"]
# CMD ["apt-get", "install", "-y", "curl"]
CMD ["./warp_client"]

