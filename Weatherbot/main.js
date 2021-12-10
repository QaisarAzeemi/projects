//API integration technique
//we will use promise and callbacks
//Promise((resolve,reject)=>{})
//in request() we will use call back that will accept 3 parameters 1)error 2)response 3)body
//request('API URL',(error,response,body)=>{ .... })
const request = require('request');

function temprature(city) {
    let promise = new Promise((resolve, reject) => {
        request(`http://api.weatherstack.com/current?access_key=c9ece71eaf12927568dca65b098f78be&query= ${city_name}`, (error, response, body) => {
            console.log(`Error,${error}`);
            console.log(`Response,${response}`);
            console.log(`Body,${body}`);
            resolve(body);
        })
    })
    return promise
}
var city_name = "Rabat"
temprature(city_name)
    .then(temp => {
        console.log(temp)
        var temp_data = JSON.parse(temp);
        console.log("Arranged Data Object", temp_data)
        console.log(`Temperature of ${city_name} is ` + temp_data.current.temperature + "'C")
        console.log("Visibility =" + temp_data.current.visibility + "m")
        console.log(`In ${city_name} is there Day time? :` + temp_data.current.is_day)
    })
    .catch(() => {
        console.log('Error in getting Data')
    })