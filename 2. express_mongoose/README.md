# Mongoose with Docker example

This is a simple REST API example with Express and Mongoose. I made this example to prototype the Rust project. It will work with Rust Email CLI made with **Lettre**.

## How to install dependencies

1. Install mongodb with these Docker commands.

```console
$docker volume create mongodbdata
$docker run -d -v mongodbdata:/data/db --name mongo -p 27017:27017 mongo 
```

2. Then, Install the npm packages with **$yarn or npm install**
3. Start the Express server with **$yarn dev or node server.js**.

## How to test the REST CRUD end points with CURL

1. Register an email

```console
$curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "steady@learner.com" }'
$curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "example@email.com" }'
```

2. Read it

```console
$curl -X GET localhost:8000/api/email/v1
```

3. Update it

```console
$curl -X PUT localhost:8000/api/email/v1/steady@learner.com -H "Content-Type: application/json" -d '{ "response": "true" }'
```

4. Delete it

```console
$curl -X DELETE localhost:8000/api/email/v1/steady@learner.com
```

5. List emails

```console
$curl -X GET localhost:8000/api/email/v1
```
