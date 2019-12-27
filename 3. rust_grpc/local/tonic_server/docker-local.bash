# Remove the previous container if there were any.
docker rm tonic_server -f 
# 1. Use this(--nework="host") to use Postgresql datas in local machine.
docker run --network="host" -d --name tonic_server steadylearner/tonic_server

# 2. Use this with docker-network.bash if it were not for 1.
# docker run -d -p 50051:50051 --name tonic_server steadylearner/tonic_server


