const { WebhookClient } = require('dialogflow-fulfillment');
const express = require('express');
const app = express();

app.post("/webhook", express.json(), (request, response) => {
    const agent = new WebhookClient({
        request: request,
        response: response
    })

    function welcome(agent) {
        agent.add(`welcome to testbot from backend by Qaisar`)
        var num01 = agent.parameters.number01;
        var num02 = agent.parameters.number02;
    }
    let intentMap = new Map();
    intentMap.set('Default Welcome Intent', welcome);
    agent.handleRequest(intentMap)
})
app.listen("8085", () => {
    console.log("Server is Up");
})