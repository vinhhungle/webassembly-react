const express = require("express");
const path = require("path");
const app = express();

const port = 8081;

express.static.mime.types["wasm"] = "application/wasm";

app.use(express.static(path.resolve(__dirname, "./build")));

app.listen(port, () => {
  console.log(`http://localhost:${port}`);
});
