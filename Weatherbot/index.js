const { webhookClient } = require('dialogflow-fulfillment')
const express = require('express')
const bodyParser = require('body-parser')

var app = express();
app.use(bodyParser.json());
var port = process.env.PORT || 8000
app.post('/weather', (request, response) => {
    const agent = new webhookClient({ request: request, response: response })

    let intentMap = new Map();
    intentMap.set('weather', tempCity);
    agent.handleRequest(intentMap);

    function tempCity(agent) {
        console.log(agent.request);
        var city_name = agent.parameters.geo - city;
        agent.add(`Temp_Check, ${city_name}`);
    }
})
app.listen(port, () => {
    console.log("Server is UP......");
})