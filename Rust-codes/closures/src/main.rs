
// page # 286 of new book
// Closures are anonymus funtions.... means functions that don't have any name\
// closures can be stored in variables
// Syntex :  let variable = || {};

#![allow(unused_variables)]
fn main() {
    let closure_variable = || {
        println!("I am an anonymus Function.");
    };
//........defining parameteres and passing variables in closures...............
    let var = |x| {
        println!("{}",x);
    };
    //println!("Hello, world!");
    closure_variable();
    var(10);
}
