# Rust Full Stack Developer Resume

This is to help others to test the example projects from [Steadylearner](https://www.steadylearner.com). The easiest one to test will be **1. react_responsive** and will be simialr to this.

![user-signup](https://www.steadylearner.com/static/images//post/React/user-signup.png)

I have spent a decent amount of time for React development. But, I currently have more interest in server side development and the databases. **2. express_mongoose**, **3.rust_grpc**, **4.express_auth_redis_postgres** is to show some examples for that.

I already deployed some projects to AWS with Docker and DigitalOcean with Nginx and Linux.

For this repository, I could make [Rust Warp REST end points, Tonic gRPC server, Postgresql from AWS RDS and Redis wrapper for it](54.210.186.245/api/user/v1) deployed at AWS. You can verify the entire project at **3.rust_grpc**.

I also plan to make Rust version of **2.** and inlcude it to **3.** later with [mongodb crate](https://crates.io/crates/mongodb)
and write [blog posts](https://www.steadylearner.com/blog/search/Rust) for it later.

## How to test it

I let **README.md** for each directory. Use **$git checkout deploy** after you clone this repository and follow the instructions.

## Temporary available project

**3.rust_grpc** is currently availiable at AWS with ECS Fargate option. You can see that it is learnign with this log.

```console
tonic_server  RUNNING                             
warp_client   RUNNING  54.210.186.245:80->80/tcp
redis         RUNNING                             
``` 

You can test it easily with **$curl 54.210.186.245/api/user/v1**. The result will be similar to this.

```json
{"users":[{"full_name":"$argon2i$v=19$m=4096,t=3,p=1$tub6PYcVFuW5THuRbM25DwtljuHcQHFHH0hteD2Kgw4$n3XcdvL6pTwfnCp2TEMPI4/QiqM6+myvmCdfi2ltvXw"},{"full_name":"$argon2i$v=19$m=4096,t=3,p=1$jNLxlX2S2mkDig+69zCujt7SwBCNzL5FLUiy6f5o5Bk$4NcyOrFXlg0TKfCO52rA+1J3zhAeegkbRJV25jEJsKA"},{"full_name":"$argon2i$v=19$m=4096,t=3,p=1$b+T4XftG29CZnDZpoUgvwUkuzmUZdlCOiwZPN0s0Wco$Vfn+VWKq1KGpUxjCitBymFZp41uBpZ7B49gpPVNC4GQ"},{"full_name":"$argon2i$v=19$m=4096,t=3,p=1$gXbryTAxdRbaCaycCGGK59ZJ7Xum/NgqbxY0j9u96oo$L3qBRRsr+uSTnp9R3Cw9kxFHOWHuJEZroExhhQBaKyE"}]}
```

It will return hashed **full_name** from **first_name** and **last_name** data from a user in **Postgresql** database cached by **Redis**.

You can get a data for the specific user also. For example, **$curl 54.210.186.245/api/user/v1/steadylearner**.

It will return the data simialr to this.

```json
{"full_name":"$argon2i$v=19$m=4096,t=3,p=1$8opynhrDw36tn5eCJ1Gho72HwicjxLzNfOMIzfTWW3E$h+Pxzh8F7n/7o3gURErRIZt6IjYKeIP89AILoDSrbXo"}
```

If you want to test more commands, please refer to **3.rust_grpc** folder. I don't let delete or update commands here not make other easily delete datas before people in your company test the end points.

## More examples

You can visit [Rust Full Stack](https://github.com/steadylearner/Rust-Full-Stack) repository. You can easily test a full stack Rust chat app example made with Yew and Rocket.

It will be simialr to this.

![rust-chat-app](https://camo.githubusercontent.com/d7c7d4f5072c5c5246995fab96f7bd905eadd068/68747470733a2f2f7777772e7374656164796c6561726e65722e636f6d2f7374617469632f696d616765732f706f73742f7765622f66756c6c2d737461636b2d727573742d636861742d6170702d62792d7374656164796c6561726e65722e706e67)

You can also read [blog posts at Steadylearner](https://www.steadylearner.com/blog/search/Rust).

![rust-full-stack-code](https://camo.githubusercontent.com/4b453b650482d4b79951972cab1d5ce11ff0268e/68747470733a2f2f7777772e7374656164796c6561726e65722e636f6d2f7374617469632f696d616765732f706f73742f7765622f636c69656e742d7365727665722d657175616c2d727573742d636f64652e706e67)
