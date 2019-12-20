# Use this to locally verify it all micro services work before you deploy them to AWS. 

docker network create express_mongoose
docker network ls
docker network connect express_mongoose mongo
docker network connect express_mongoose express_mongoose 
docker network inspect express_mongoose

# You should also modify 0.0.0.0 to mongo to verify these work in local environment
# Modify it in config/default.json or .env with $docker exec -it express_mongoose bash
# "mongoURI": "mongodb://0.0.0.0:27017/email" to "mongoURI": "mongodb://mongo:27017/email"
