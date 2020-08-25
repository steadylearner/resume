# Make a Rust production file first.
# cargo run --release
# Remove the previous image.
docker rmi steadylearner/tonic_server:aws -f 
docker build -t steadylearner/tonic_server:aws .
docker push steadylearner/tonic_server:aws
