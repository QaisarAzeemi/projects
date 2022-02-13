// const express = require('express')
// const request = require('request');
// const bodyParser = require('body-parser')

// const app = express();
// app.use(bodyParser.json());
// var port = process.env.PORT || 3030
// exports.myFunction = (req, res) => {
//     res.send(`<h1>You're awesome 🤘</h1>`);
// };
// app.listen(port, () => {
//     console.log("Server is UP......");
// })
const { dialogflow } = require('actions-on-google');
const express = require('express');
const { createHandler } = require('azure-function-express');
const app = dialogflow();
app.intent('Default Welcome Intent', conv => {
    // conv.close('Hello, Azure!');
    conv.add("Hello This is MicroSoft BackEnd")
});
// Put other intent handlers here.
const expressApp = express();
expressApp.post('/api/appFunc01', app);
module.exports = createHandler(expressApp);

