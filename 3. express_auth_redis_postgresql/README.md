# Express Passport Login Example

This is a simple project to test Node passsport API. When you visit /register or /login page, you will see the page simialr to this from **1.react_responsive**.

![user-signup](https://www.steadylearner.com/static/images//post/React/user-signup.png)

No frontend frameworks are used here. We don't need frontend parts to test auth parts. But, I include them to make it more meaningful and easy to develop than to use CURL commands.

# How to test it

I will assume that you are already familar with Docker. If not, please read these blog posts.

1. [How to use Docker commands](https://www.steadylearner.com/blog/read/How-to-use-Docker-commands)
2. [How to use Docker with Rust](https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust)

First, install Redis and Mongodb with Docker.

```console
$docker volume create mongodbdata
$docker run -d -v mongodbdata:/data/db --name mongo -p 27017:27017 mongo
$docker run -d --name redis -p 6379:6379 redis
```

Then, Install the npm packages with **$yarn or npm install**.

Start the Express server with **$yarn dev or yarn serve**. When you visit http://localhost:8000, it will be empty page with navbar. the / route is used to redirect users when necessary.

Test its main api at /login and /register.


