# To use localhost
docker run -d --network="host" --name="redis" redis
# To use network
# docker run -d -p 6379:6379 --name="redis" redis
