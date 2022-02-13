#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#[derive(Debug)]
enum Coin{  // USD
    Penny,
    Nickle,
    Dime,
    Quarter(UsStates),
    duo
}
#[derive(Debug)]
enum UsStates{
    Alaska,
    Albama
    //.......etc....
}

fn coin_sort(coin: Coin) -> u32 {

    match coin {
        Coin::Penny => { println!("Nice! you won Penny.."); 1 },
        Coin::Nickle => { println!("Good! you won Nickle.."); 5},
        Coin::Dime => {println!("Wow! you won Dime.."); 10},
        Coin::Quarter(kon) => {println!("Great! you won Quarter of {:?} State..", kon); 25},
        _ => 111,
        
    }
}

fn main() {

let received : u32 = coin_sort(Coin::Quarter(UsStates::Alaska));
    println!("Received Coin is {}", received);
    let received = coin_sort(Coin::duo);
    println!("Wrong entry {} .... !", received);

    //....Using Option enum for match CFO..............................................................
    let  four = Some(4);
    println!("{:?}", four);
    let value = Plus_One(four);
    println!("{:?}", value);
    let extract : f64 = match value {
        Some(i) => i as f64,       // return type of all match arms must be same
        None => {println!("Nothing to display.......!"); 0 as f64},
    };
    println!("{}",extract);
    let no_value = Plus_One(None);
    println!("{:?}", no_value);
}

fn Plus_One(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,  // if we don't write None, we will get error ... see page 139 of old book
        Some(i) => Some(i+1),
        //_ => () can't write this because it needs to return option<i32>
    }
}