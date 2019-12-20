// https://www.freecodecamp.org/news/how-to-write-a-production-ready-node-and-express-app-f214f0b17d8c/
const express = require("express");
const bodyParser = require("body-parser");

const app = express();

const { user } = require("./routes");

// Use this to verify a Docker container was deployed well.
// app.get("/hello", (_req, res) => {
//     res.send("hello");
// });

app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

app.use("/api/user/v1", user)

module.exports = app;
