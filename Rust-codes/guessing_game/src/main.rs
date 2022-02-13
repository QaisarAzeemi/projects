extern crate rand;
use std::io;     // enabling input output console from standard library
use std::cmp::Ordering;    // Using comparision function from standard library
use rand::Rng;           // using random number generation function from downloaded crate rand 
fn main() {
   loop 
   {
    let mut guess = String::new();  // declearing variable to get input from user

    println!("Its guessing game.");

    println!("Enter your guess:");
    io::stdin().read_line(&mut guess).expect("failed to read line"); // accepting string input from user
    let guess : u32 = match guess.trim().parse() { //.expect("Please type a number."); // string to integer conversion
    Ok(num) => num,
    Err(_) => continue,  // continue to the program even on wrong entry
    };
    println!(" You Guessed : {}", guess);

    let secret_num = rand::thread_rng().gen_range(1,101); // generating random number using rand crate
    println!("The secreat number is: {}", secret_num);
    
    match guess.cmp(&secret_num) {                  // comparision
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Large"),
        Ordering::Equal => { println!("You Won! :)");
        break;
        }
    }

   } 
}
