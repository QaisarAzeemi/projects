var mqtt = require('mqtt')
var client = mqtt.connect('mqtt://192.168.18.54')
client.on('connect', function() {
    client.subscribe('onoff');
})
client.on('message', function(topic, message) {
    context = message.toString();
    console.log(context)
})