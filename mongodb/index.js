const express = require("express");
const app = express();
app.use(express.json());
const port = process.env.PORT || 2022;

app.listen(port, () => {
    console.log(`Server is UP and running and listening at port ${port}`);
})