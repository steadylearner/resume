const chalk = require("chalk");
const app = require("./server");

const { PORT = 8000 } = process.env;

app.listen(PORT, () => {
  const blue = chalk.blue
  const target = blue(`http://0.0.0.0:${PORT}`)
  console.log(`🚀 Express Server ready at ${target}`)
})