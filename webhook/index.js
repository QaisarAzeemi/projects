//portal CLASS_11 .... 1:20:00
//https://pure-tor-98713.herokuapp.com/webhook
const { WebhookClient } = require('dialogflow-fulfillment');
const express = require('express');
const bodyParser = require('body-parser');
//const { request, response } = require('express');
const app = express();
//app.use(express.json());
//app.use(bodyParser.urlencoded({ extended: false }));
//app.use(bodyParser.json()); //this is also true and must be added to program
port = process.env.PORT || 8080
    // app.get("/", (request, response) => {
    //     response.send("This is the first calculator app on Dialogflow");
    // })
    // app.get('/index', () => {
    //     response.sendFile(__dirname, 'index.html')
    // })
app.post("/webhook", bodyParser.json(), (request, response) => {
    const agent = new WebhookClient({ request: request, response: response })

    function calculate(agent) {
        console.log(agent.request);
        var num01 = agent.parameters.number01;
        var num02 = agent.parameters.number02;
        agent.add(`welcome to calculatorbot from backend by Qaisar, ${num01} + ${num02} = ${num01+num02}`);
    }
    let intentMap = new Map();
    intentMap.set('Calculate', calculate);
    agent.handleRequest(intentMap)
})
app.listen(port, () => {
    console.log("Server is Up");
})