//writing a server code
//This code is implemented in heroku in heroku deployment project
const { response } = require("express");
const express = require("express"); // calling the express module
const app = express(); //putting functionality of express module to express() function and saving in app variable or constant
//const port = process.env.PORT || 8082

var appliances = [
    { id: 1, name: "Juicer", quantity: 3 }, { id: 2, name: "Washig machine", quantity: 2 },
    { id: 3, name: "owen", quantity: 1 }
];

app.use(express.json()) // middlewear called to interect with outer world via post. i will connect your machine 
    // with the server

app.get("/appliances", (request, response) => {
    //response.send("This is the list of students " + JSON.stringify(students));
    var result = `<table border=2 >
                    <tr>
                    <th>SN</th>
                    <th>NAME</th>
                    <th>Quantity</th>
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
app.post("/appliances", (request, responses) => {
    var appliance = {
        id: appliances.length + 1,
        name: request.body.name,
        quantity: request.body.quantity
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

app.get("/student/:year/:month/:day", (request, response) => {
    response.send("Year = " + request.params.year + " Month = " + request.params.month + " Day = " + request.params.day)
})

app.get("/practice", (request, response) => {
    response.send("This is Node JS server code API practice program");
})

app.get("/", (request, response) => {
    response.send("Wecome to the Home page");
})

app.listen(8082, () => {
    console.log(`The server is up and running on port 8082`);
})