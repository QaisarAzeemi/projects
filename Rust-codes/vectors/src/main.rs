// Syntax     Vec<T>    .... for further details click the following link
// https://doc.rust-lang.org/stable/nomicon/vec-layout.html 
//Vectors are just like arrays .. both store homogeneous data
//Major difference between vectors and arrays is that 
// Arrays are Static.... values can't be added or subtracted once they are decleared
// while ... Vectors are Dynamic... we can push into and pop out of the values from Vectors
// array data is stored on Stack while Vector data is stored on Heap

#![allow(unused_variables)]
#![allow(dead_code)]
mod  lib;
fn main() {
let mut v1 : Vec<i32> = vec![10,20,30,40,50]; // vector stored using Rust macro
let mut v2 : Vec<i32> = Vec::new(); // Defining vector using Vec::new() Associated function
v1.push(60);
v2.push(60);    
    println!("{:?}", v1);
    lib::see();
    println!("{:?}", v2);
// Displaying an Array
    let a1 = [1,2,3,4,5,6,7];
//a1.push(8); // Arrays are static .. so its values can't be pushed or popped.. this line will throw and error
println!("{:?}", a1);
    // Accessing a Vector
let c = v1[1]; // accessing a vector is just like array
println!("{}", c);
println!("{:?}", v1);
//let g = v1[100]; //Rust will panic.. program will be executed but not successfully

let d = &v1[2]; //Immutable borrow
//v1.push(32);    //mutable borrow.... it will throw the error due to borrowing rules

println!("{}", d);
println!("{:p}", d); // Displaying address of variable d

let e = v1.get(3); // Accessing a vector value using get function will return through option<&T> enum 
println!("{:?}", e);
let f = v1.get(10); // advantage of useing get function to access vector
println!("{:?}", f);

match f {
    Some(value) => println!("{}", value),
    None => println!("NO value, index beyond vector length...!!!")
}

// Itterating vector elements using for loop
/*
for i in v1 {   // ownership of vector v1 is passed here
    println!("{}", i);
}
println!("{:?}", v1); // so this will not work and print an error
*/

for i in &v1 {   // borrowed v1 to prevent passing ownership of vector v1 
    println!("{}", i);
}
println!("The vector v1 is \n {:?}", v1); // This will work now and print the vector

println!("\nNow adding 35 to each value of vector v1");

for i in &mut v1{
    *i += 35; // we must dereference it using derefrencing operator (*)
}

println!("{:?}", v1);

println!("\nWe will store different data type values in a new vector v4 using enum....!!!");
let v4 = vec![SpreadSheetCell::Class_number(1), SpreadSheetCell::Name(String::from("Qaisar")), SpreadSheetCell::Percentage(92.67)];
println!("Student data in v4 is \n{:?}",v4);
// infact we are going to store UDDT enum in the vector v4
let data1= spread {
    Class_number : 2,
    Name : String::from("Azeemi"),
    Percentage : 65.1324
};
let data2= spread {
    Class_number : 3,
    Name : String::from("Azher"),
    Percentage : 72.249
};
let v5 = vec![data1, data2];//.Class_number, data1.Name, data1.Percentage];
println!("Student data in v5 is \n{:#?}",v5);

}

#[derive(Debug)]
enum SpreadSheetCell {
    Class_number(i32),
    Percentage(f64),
    Name(String)
}

#[derive(Debug)]
struct spread {
    Class_number : i32,
    Percentage : f64,
    Name : String
} 