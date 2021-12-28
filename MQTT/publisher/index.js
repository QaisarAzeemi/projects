var mqtt = require('mqtt');
var client = mqtt.connect('mqtt://192.168.18.54');
client.on('connect', function() {
    setInterval(function() {
        //client.publish('onoff', '0 (Qaisar)');
        client.publish('onoff', '0');
        console.log('Message Sent');
    }, 5000); //sending message after 5 seconds
});
//broker mae "npm i mosca"
//publisher mae "npm i mqtt"
//npm i jsonschema@1.2.6


//WE CAN USE PlatformIO in vscode as an extention instead of ardrino

// client.on('connect', function() {
//     //client.publish('onoff', '0 (Qaisar)');
//     client.publish('onoff', '1');
//     console.log('Message Sent'); //sending message after 5 seconds
// });