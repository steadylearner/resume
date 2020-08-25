# 1. Redis

# To use localhost
docker run -d --network="host" --name="redis" redis
# To use network
# docker run -d -p 6379:6379 --name="redis" redis

# 2. Postgresl

# https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_ConnectToPostgreSQLInstance.html
# Save this in ~/.bashrc
# alias rds="psql --host=<aws> --port=5432 --username=postgres --password --dbname=postgres"

# If you see this message,

# Find where subnet belongs at https://us-west-2.console.aws.amazon.com/rds/home?region=us-west-2#database:id=rustgrpc;is-cluster=false
# Verify which security group belongs https://us-west-2.console.aws.amazon.com/rds/home?region=us-west-2#database:id=rustgrpc;is-cluster=false

# You should help your security group that connected to RDS(firewall) to allow this port.
# psql: could not connect to server: Connection timed out
#	Is the server running on host <aws> accepting
#	TCP/IP connections on port 5432?

# You have to first enable this publically accessible to test the database.
# https://us-west-2.console.aws.amazon.com/rds/home?region=us-west-2#databases:

# If there were no security group,
# -> Create Security group at https://us-west-2.console.aws.amazon.com/vpc/home?region=us-west-2#SecurityGroups:
#    with vpc from https://us-west-2.console.aws.amazon.com/rds/home?region=us-west-2#subnet-group:ids=default-vpc-04c34422e5d20c76d
