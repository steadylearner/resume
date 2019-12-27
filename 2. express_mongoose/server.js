const chalk = require("chalk");
const express = require('express');
const app = express();
const useMongo = require("./db/mongoose");
useMongo();

const {
  email,
} = require("./routes");

app.use(express.json());
app.use("/api/email/v1", email);

const { 
    PORT = 8000, 
} = process.env;

app.listen(PORT, () => {
  const blue = chalk.blue;

  const target = blue(`http://0.0.0.0:${PORT}`);
  console.log(`🚀 Server ready at ${target}`);
});
