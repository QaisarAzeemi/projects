var a = 5;
var b = 7;
if (a < b) {
    console.log("a is less than b")
} else {
    console.log("b is less than a")
}
op = '+'
switch (op) {
    case '+':
        console.log(a + b);
        break;

    case '-':
        console.log(a - b);
        break;

    case '*':
        console.log(a * b);
        break;

    case '/':
        console.log(a / b);
        break;

    default:
        console.log("Invilad operator");
}

// for (i = 0; i < 6; i++) {
//     console.log(i)
// }
// i = 0;
// while (i < 7) {
//     console.log(i)
//     i++
// }
// i = 1;
// do {
//     console.log(i)
//     i++
// } while (i < 9);
//var i = 0;
arr = [1, 20, 300, 4000, 50000, 600000];
// for x in arrData:
//     // console.log(i);
//     print(i);
arr.forEach(i => {
    console.log("\n", i);
});
arr2 = ["Fahim", "Naeem", "Saleem"]
for (i = 0; i < arr2.length; i++) {
    console.log("b = ", arr[i])
    console.log("form for " + arr2[i])
    console.log(`second method ${ arr2[i]}`)
}

function printFunc() {
    console.log("\nPrint the text only")
}

function printFuncWitParam(text) {
    console.log("\nPrint function with " + text)
}

function printFuncWitReturn(text) {
    //console.log("\nPrint function with " + text)
    return "Return Value" + text
}
printFunc()
printFuncWitParam("Parameter")
console.log(printFuncWitReturn(" With Parameter"))
    //Since Java Script is a Dynamically Typed language, we can use multiple data types in an array
arrData1 = ["Qaisar", 37, 3.8, 'qaisr@yahoo.com'] // array
arrData2 = [
    ["Qaisar", 37, 3.8, 'qaisr@yahoo.com'],
    ["Farhan", 41, 2.8, 'farhan@yahoo.com'],
    ["Raju", 8, 1.8, 'raju@yahoo.com']
]
for (i = 0; i < arrData2.length; i++) {
    for (j = 0; j < arrData2[i].length; j++) {
        console.log(arrData2[i][j])
    }
}
var obj1 = { // object in JS...... Just like structure
    "name": "Qaisar",
    "age": 37,
    "GPA": 3.5,
    "email": "mqa@hotmail.com"
}
console.log(obj1.email)
console.log("Task01");
setTimeout(() => { console.log('Task02') }, 3000); // arrow functions are just like clousers in RUST
console.log("Task03")

function add(x, y) {
    return x + y
}
x = add //(4, 5)
console.log(x(4, 5))
console.log(x)
var g = (x, y, z) => { // closure type functions or arrow functions
    return (x + y + z)
}
console.log(g(4, 7, 9))
arrData3 = ['task04', 'task05', 'task06'] // structural programming
for (let i = 0; i < arrData3.length; i++) {
    element = arrData3[i] + " Completed"
    console.log(element);
}
console.log(arrData3);
var arrData3CallBack = arrData3.map((value) => { // functional language
    return value + " also Done";
    // console.log(value + "also Done");
});
console.log(arrData3CallBack)
var check = false;
var prom = new Promise((resolve, reject) => { //used in server side API integration
    if (check) {
        resolve("Promised Task Completed ")
    } else {
        reject("Error in Task completion")
    }
});
//console.log(prom);
prom
    .then(() => {
        console.log("Task Done")
    })
    .catch(() => {
        console.log("Task Crashed")
    })

async function testasync() { // Asynchronous programming
    try {
        let result = await prom;
        console.log(result);
    } catch {
        console.log("Async Task Crashed")

    }
}
testasync();