const express = require("express");
var app = express();
const bodyParser = require(`body-parser`)

var port = process.env.PORT || 8088
app.use(bodyParser.urlencoded({ extended: false }))
app.use(bodyParser.json())
app.use(express.json());
app.use(express.static('web.html'));
var appliances = [
    { id: 1, name: "Juicer", quantity: 3 },
    { id: 2, name: "Washig machine", quantity: 2 },
    { id: 3, name: "owen", quantity: 1 }
];

app.use(express.json()) // middlewear called to interect with outer world via post. i will connect your machine 
    // with the server
app.get("/web", (request, response) => {
    response.sendFile(__dirname + "/web.html")
})
app.get("/appliances", (request, response) => {
    //response.send("This is the list of students " + JSON.stringify(students));
    var result = `  <table border = 2>
                    <tr>
                    <th> SN </th>
                    <th> NAME </th>
                    <th> Quantity </th>
                    </tr>`
    appliances.map((value) => {
        result += `<tr>
                    <td>${value.id}</td>
                    <td>${value.name}</td>
                    <td>${value.quantity}</td>
                    </tr>`
    })

    response.send(result)
})
app.post("/postappliances", (request, response) => {
    var appliance = {
        id: appliances.length + 1,
        name: request.body.name,
        quantity: request.body.quantity
    }
    appliances.push(appliance)
    response.send("Data is added to the main streem")
})
app.get("/appliances", (request, response) => {
    var appliance = {
        id: appliances.length + 1,
        name: request.query.name,
        quantity: request.query.quantity
    }
    appliances.push(appliance)
    response.send("Data is added to the main streem")
})

app.put("/appliance/:id", (request, response) => {

    var appliance = appliances.find(i => i.id === parseInt(request.params.id))
        // var index = students.indexOf(student)
        // students.splice(index, 1)
    response.send("The id is this : " + appliance)
    appliance.name = request.body.name
    response.send("Index is Updated")
})

app.delete("/appliance/:id", (request, response) => {

    var appliance = appliancs.find(i => i.id === parseInt(request.params.id))
    var index = appliancs.indexOf(student)
    appliancs.splice(index, 1)
    response.send("Record is Deleted")
})

app.get("/", (request, response) => {
    response.send("This is my first Heroku deployment app running on port " + port);
})
app.listen(port, () => {
        console.log("The server is up and running on port " + port);
    })
    // console.log("The server is up and running on port " + port);
    // })