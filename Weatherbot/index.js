//Class 14
//Weather API
//https://hidden-headland-23965.herokuapp.com/weather
//http://api.weatherstack.com/current?access_key=c9ece71eaf12927568dca65b098f78be&query=Peshawar
const { WebhookClient } = require('dialogflow-fulfillment')
const express = require('express')
const request = require('request');
const bodyParser = require('body-parser')

const app = express();
app.use(bodyParser.json());
var port = process.env.PORT || 8080
app.post("/weather", (request, response) => {
    const agent = new WebhookClient({ request: request, response: response })

    let intentMap = new Map();
    intentMap.set('weather', tempCity);
    agent.handleRequest(intentMap);

    function tempCity(agent) {
        // console.log(agent.request);
        var city_name = agent.parameters["city"];
        //  agent.add(`Temp_Check ${city_name}`);
        return temprature(city_name) //calling function here
            .then(temp => {
                console.log(temp)
                var temp_data = JSON.parse(temp);
                console.log("Arranged Data Object", temp_data)
                return agent.add(`Temperature of ${city_name} is ` + temp_data.current.temperature + "'C")
                return agent.add("Visibility =" + temp_data.current.visibility + "m")
                return agent.add(`In ${city_name} is there Day time? :` + temp_data.current.is_day)
            })
            .catch(() => {
                return agent.add('Error in getting Data..........')
            })
    }

})
app.listen(port, () => {
    console.log("Server is UP......");
})

function temprature(city) {
    let promise = new Promise((resolve, reject) => {
        request(`http://api.weatherstack.com/current?access_key=c9ece71eaf12927568dca65b098f78be&query=${city}`, (error, response, body) => {
            console.log(`Error,${error}`);
            console.log(`Response,${response}`);
            console.log(`Body,${body}`);
            resolve(body);
        })
    })
    return promise
}


//.......
// const { WebhookClient } = require('dialogflow-fulfillment')
// const express = require('express')
// const bodyParser = require('body-parser')
// const request = require("request")
// const app = express();

// app.use(bodyParser.json())
// var port = process.env.PORT || 8088

// app.post("/webhook", (request, response) => {
//     const agent = new WebhookClient({ request: request, response: response })

//     function welcome(agent) {
//         console.log(agent.request);
//         var num_one = agent.parameters.num01;
//         var num_two = agent.parameters.num02;
//         agent.add(`Welcome to PIAIC bot from Backend ${num_one} ${num_two} ${num_one+num_two}`)
//     }

//     function temp_check(agent) {
//         // console.log(agent.request);        
//         var city_name = agent.parameters["geo-city"];
//         console.log(city_name)
//             // agent.add(`Temp_Check ${city_name}`)
//         return temperature(city_name)
//             .then(temp => {
//                 console.log(temp)
//                 var data_temp = JSON.parse(temp);
//                 console.log(data_temp)
//                 return agent.add(`The Temp of ${city_name} is this ${data_temp.current.temperature}'C`)
//             })
//             .catch((err) => {
//                 return agent.add(`Error in Fetching the results ${err}`)
//             })
//     }

//     let intentMap = new Map();
//     intentMap.set('Calculate', welcome);
//     intentMap.set('Temp-City', temp_check);
//     agent.handleRequest(intentMap)
// })

// function temperature(city) {
//     let promise = new Promise((resolve, reject) => {
//         request(`http://api.weatherstack.com/current?access_key=35fbfe7009e2447c893bb93594852617&query=${city}`, (error, response, body) => {
//             console.log(`Error ${error}`)
//             console.log(`Response ${response}`)
//             console.log(`Body ${body}`)
//             resolve(body)
//         })
//     })
//     return promise
// }

// app.listen(port, () => { console.log("Server is Up") })
//https://enigmatic-harbor-75799.herokuapp.com/webhook