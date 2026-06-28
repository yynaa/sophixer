import { sleep } from "bun";
import "./build";
import * as express from "express";

sleep(500).then(() => {
  const app = express.default();
  const port = 3000;

  app.use(express.static("./dist"));

  app.listen(port, "0.0.0.0", () => {
    console.log("server running");
  });
});
