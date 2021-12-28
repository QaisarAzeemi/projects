//portal CLASS_17 .... 
//https://shielded-stream-03167.herokuapp.com/
const { WebhookClient } = require('dialogflow-fulfillment');
const express = require('express');
const bodyParser = require('body-parser');
var mqtt = require('mqtt');
var client = mqtt.connect('mqtt://192.168.18.54');
//const { request, response } = require('express');
const app = express();
//app.use(express.json());
//app.use(bodyParser.urlencoded({ extended: false }));
//app.use(bodyParser.json()); //this is also true and must be added to program
port = process.env.PORT || 8077
app.get('/', (req, res) => {
    res.send(`Heroku app is deployed on port : ${port}`);
})
app.post("/vc", bodyParser.json(), (request, response) => {
    const agent = new WebhookClient({ request: request, response: response })

    // function calculate(agent) {
    //     console.log(agent.request);
    //     var num01 = agent.parameters.number01;
    //     var num02 = agent.parameters.number02;
    //     agent.add(`welcome to calculatorbot from backend by Qaisar, ${num01} + ${num02} = ${num01+num02}`);
    // }
    function turnOn(agent) {
        console.log(agent.request);
        client.publish('onoff', '1');
        agent.add(`Turning ON the LED`);
    }

    function turnOff(agent) {
        console.log(agent.request);
        client.publish('onoff', '0');
        agent.add(`Turning OFF the LED`);
    }
    let intentMap = new Map();
    //intentMap.set('Calculate', calculate);
    intentMap.set('Turn ON', turnOn);
    intentMap.set('Turn OFF', turnOff);
    agent.handleRequest(intentMap)
})
app.listen(port, () => {
    console.log("Server is Up and running on port : " + port);
})