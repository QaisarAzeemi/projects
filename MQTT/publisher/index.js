var mqtt = require('mqtt');
var client = mqtt.connect('mqtt://118.103.233.199');
client.on('connect', function() {
    setInterval(function() {
        client.publish('onoff', '0 (Qaisar)');
        // client.publish('onoff', 'Turn on the light');
        console.log('Message Sent');
    }, 5000);
});
//broker mae "npm i mosca"
//publisher mae "npm i mqtt"
//npm i jsonschema@1.2.6


//WE CAN USE PlatformIO in vscode as an extention instead of ardrino