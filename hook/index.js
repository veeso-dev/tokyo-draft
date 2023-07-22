const express = require("express");
const app = express();

app.use(express.json());

app.post("/api", (req, res) => {
  const body = req.body.body;
  const metadata = req.body.metadata;
  const html = Buffer.from(body, "base64").toString("utf-8");
  console.log(html);
  console.log(metadata);

  res.end();
});

app.listen(3020, () => {
  console.log("hook mock started listening to port 3020");
});
