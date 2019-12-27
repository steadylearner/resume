# Make a Rust production file first.
# cargo run --release
# Remove the previous image.
docker rmi steadylearner/warp_client -f
docker build -t steadylearner/warp_client .
