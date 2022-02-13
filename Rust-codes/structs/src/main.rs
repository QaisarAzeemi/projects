 #[derive(Debug)] // Display format specifier

struct Book {     // structure definition
    name : String,    // key : key value
    author: String,
    price: u16,
    availability: bool,
}

fn main() {
    println!("Hello, Structs!");

    let mut book1 = Book {          // struct instance .... book1 is an object ... i.e; user defined data type
        name: String::from("Quran"),
        author: String::from("Allah"),
        price: 5000,
        availability: true, 
    };
     println!("{:#?}", book1);
     
     book1.name = String::from("Quran e Hakeem"); // changing key value
     println!("{:#?}", book1);

     let mut book3 = Book {
        name: String::from("Torat"),
        ..book1         // other values are same as book1
      };
      book3.price = 4500; // changing key value
    
     let book2 = mine(String::from("Bible"), String::from("God"));
     println!("{:#?}", book2);

     
    println!("{:#?}", book3);

}

fn mine(name:String, author:String) -> Book {
     Book {         //Shorthand syntex if field and parameter names are same .. means... key name and key value are same
        name,
        author,
        price: 5000,
        availability: true, 
    }
    
} 

//............. ASSIGNMENT # 6 .............................................................................................
//..........................................................................................................................
/*
#[derive(Debug)]

struct city {
    name: String,
    area: u32,
    population: u32
}

fn main (){
    let city_1 = city {
        name : String::from("Karachi"),
        area: 3780, //square Km
        population: 15741000
    };
    println!("{:#?}", city_1);
    println!("______________________");
    println!("{}", city_1.name);
    println!("{}", city_1.area);
    println!("{}", city_1.population);
    println!("______________________");
    println!("______________________");

    let city_2 = city {
        name: String::from("Tokyo"),
        ..city_1

    };

    println!("{:#?}", city_2);
    
    //in main function call user defined function, Print instance returned by the user defined
    //function.
    let x = String::from("Lahore");
    println!("Third city = {:#?}", directory(x));
   // println!("{}",x);

 }

    //define user defined function, User defined function should receive some arguments and
    //return an instance of your above defined struct.
    
    fn directory(city_name:String) -> city {

    let city_3 = city {
        name : city_name,
        area : 1772, //sq km
        population: 12188000
    };

    city_3


} */