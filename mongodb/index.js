const express = require("express");
const app = express();
const mongoose = require('mongoose');
app.use(express.json());
const port = process.env.PORT || 2022;
mongoose.connect('mongodb+srv://Mikrobit:qaisar1214@cluster0.ia6j1.mongodb.net/myFirstDatabase?retryWrites=true&w=majority', (err) => {
    if (err)
        console.log(err);
    else
        console.log('Connected');
});
//  A mongoose schema defines the shape of documents inside a particular collection
const userSchema = new mongoose.Schema({
    name: {
        type: String, //if we must need this data type we will use "type" and "required"
        required: true
    },
    email: {
        type: String,
        required: true
    },
    age: {
        type: Number,
        required: true
    },
}, { timestamps: true });
const User = mongoose.model("User", userSchema); // User variable is responsible to store the Data
app.get('/allusersdata', (request, response) => {
    User.find({}, (err, success) => {
        if (err) {
            return response.json(err);
        }
        return response.json(success);

    });
});
app.post('/createuser', (request, response) => {
    if (request.body.name === "" || request.body.name === undefined || request.body.name === null) {
        return response.json("Please enter the name.");
    }
    if (request.body.email === "" || request.body.email === undefined || request.body.email === null) {
        return response.json("Please enter the email.");
    }
    if (request.body.age === "" || request.body.age === undefined || request.body.age === null || typeof request.body.age !== 'number') {
        return response.json("Please enter the age or age is not a numbr.");
    }
    const newUser = new User(request.body);
    newUser.save((err, success) => {
        if (err) {
            return response.json(err);
        }
        return response.json(success);
    })
});
app.post('/updateuser', async(request, response) => {
    if (request.body.name === "" || request.body.name === undefined || request.body.name === null) {
        return response.json("Please enter the name.");
    }
    if (request.body.email === "" || request.body.email === undefined || request.body.email === null) {
        return response.json("Please enter the email.");
    }
    if (request.body.age === "" || request.body.age === undefined || request.body.age === null || typeof request.body.age !== `number`) {
        return response.json("Please enter the age or age is not a numbr.");
    }
    const doc = await User.findByIdAndUpdate(request.body._id, request.body, { new: true });
    return response.json(doc);
});
app.delete('/deleteuser/:id', (request, response) => {
    User.findByIdAndDelete(request.params.id, (err, success) => {
        if (err) return response.json(err)
        return response.json(success)
    });
});
app.listen(port, () => {
    console.log(`Server is UP and running and listening at port ${port}`);
});