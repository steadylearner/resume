# Remove the previous container if there were any.
# docker rm warp_client -f
# 1. Use this(--nework="host") to use Postgresql datas in local machine.
docker run --network="host" -d --name warp_client steadylearner/warp_client

# 2. Use this with docker-network.bash if it were not for 1.
# docker run -d -p 80:80 --name warp_client steadylearner/warp_client



