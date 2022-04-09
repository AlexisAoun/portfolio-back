import "dotenv/config";
import express from "express";
import logger from "./log/logger";

const app = express();
const port = process.env.PORT;

app.get("/", (req, res) => {
  res.send("Hello World !");
});

app.listen(port, () => {
  logger.log("info", "Server listening to port " + port);
});
