// Page # 197 of new book
// Trait definitions are a way to group method signatures together to define a set of behavior together 
// necessary to accomplish some purpose
// A trait can have multiple methods in its body, with the method
// Signatures listed one per line and each line ending in a semicolon.

#![allow(unused_variables)]
#![allow(non_camel_case_types)]

//___________________________________trait example begin____________________________
/*pub struct NewsArticle{
    pub author: String,
    pub contant: String,
}

pub struct Tweet{
    pub username: String,
    pub contant: String,
}

pub trait summary{
   fn summary(&self)->String;
}

impl summary for NewsArticle{
    fn summary(&self)->String {
        format!("{} write {}",self.author,self.contant)
    }
}

impl summary for Tweet{
    fn summary(&self)->String{
        format!("{} says {}",self.username,self.contant)
    }
}*/

//___________________________________trait example end____________________________

mod lib;
use crate::lib::NewsArticle; //accessing struct
use crate::lib::Tweet;      // accessing struct
use crate::lib::summary;    //accessing trait
use crate::lib::ending;    // accessing trait
#[derive(Debug)]
struct Superman{
    name : String,
}

#[derive(Debug)]
struct Batman{
    name : String,
}

#[derive(Debug)]
struct Hulk{
    name : String,
}

#[derive(Debug)]
struct Spiderman{
    name : String,
}

pub trait Power{
    fn power_score(&self)->u8 {50}
}

// now we are implementing this trait for sturcts
// syntex:
// impl trait for sturct
impl Power for Superman{
    fn power_score(&self)-> u8 { 100 }
}

impl Power for Batman{
    fn power_score(&self)-> u8 { 80 }
}

impl Power for Hulk{}

impl Power for Spiderman{}

fn main() {
    let my_superman = Superman {
      name : String::from("Superman"),
    };

    let my_batman = Batman {
        name : String::from("Batman"),
      };

    let my_hulk = Hulk {
        name : String::from("Hulk"),
      };

    let my_spiderman = Spiderman {
        name : String::from("Spiderman"),
      };

    println!("\n{:?}",my_superman);
    println!("Power : {}\n",my_superman.power_score());
    println!("\n{:?}",my_batman);
    println!("Power : {}\n",my_batman.power_score());
    println!("\n{:?}",my_hulk);
    println!("Power : {}\n",my_hulk.power_score());
    println!("\n{:?}",my_spiderman);
    println!("Power : {}\n",my_spiderman.power_score());
   
    println!("_______________lib trait example____________________________\n");

    let NewsArticle_1 = NewsArticle {
        author: String::from("john"),
        contant: String::from("-->A Quick Brown Fox Jumps over a lazy Dog."),
    };
    let Tweet_1 = Tweet {
        username : String::from("Qaisar"),
        contant : String::from("-->Must watch Dilirish Ertugral"),
    };
 
    println!("\n{}",NewsArticle_1.summary());
    println!("{}",NewsArticle_1.endtime());
    println!("{}",Tweet_1.summary());
    println!("{}\n",Tweet_1.endtime());

    println!("_______________Trait Bound Syntex____________________________\n");


}
