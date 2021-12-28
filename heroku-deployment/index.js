//https://mighty-temple-54237.herokuapp.com/
//Express is a minimal and flexible Node.js web application framework 
//that provides a robust set of features for web and mobile applications.
const express = require("express");
var app = express();
//Body-parser is the Node.js body parsing middleware.
//it is responsible for parsing the incoming request bodies in a middleware before you handle it.
const bodyParser = require(`body-parser`)

var port = process.env.PORT || 8088
app.use(bodyParser.urlencoded({ extended: false }))
app.use(bodyParser.json())
app.use(express.json());
app.use(express.static('web.html'));
//To serve static files such as images, CSS files, html files,
//and JavaScript files, use the express.static built - in middleware function in Express.
var appliances = [ // making array of Objects
    { id: 1, name: "Juicer", quantity: 3 },
    { id: 2, name: "Washig machine", quantity: 2 },
    { id: 3, name: "owen", quantity: 1 }
];

app.use(express.json()) // middlewear called to interect with outer world via post. i will connect your machine 
    // with the server
app.get("/web", (request, response) => {
        response.sendFile(__dirname + "/web.html")
    })
    //sendFile() function basically transfers the file at the given path and it sets the Content-Type response 
    //HTTP header field based on the filename extension.Parameter: The path parameter describes the path and
    // the options parameter contains various properties like maxAge, root, etc and fn is the callback function
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
    //send() function basically sends the HTTP response. The body parameter can be a String or a Buffer object 
    //or an object or an Array. Parameter: This function accepts a single parameter body that describe the body 
    //which is to be sent in the response.
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
    var index = appliancs.indexOf(appliance) //The indexOf() function is a string function from Node.js
        // which is used to find a string with another string.
    appliancs.splice(index, 1)
        //The splice() method is a built-in method for JavaScript Array objects. It lets you change the content of 
        //your array by removing or replacing existing elements with new ones. This method modifies the original 
        //array and returns the removed elements as a new array.
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