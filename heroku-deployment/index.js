const express = require("express");
var app = express();

var port = process.env.PORT || 8088
app.use(express.json());
app.get("/", (request, response) => {
    response.send("This is my first Heroku deployment app running on port " + port);
})
app.listen(port, () => {
    console.log("The server is up and running on port " + port);
})