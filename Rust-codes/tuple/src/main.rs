// Practicing compound data type "Tuple"

fn main() {
    
    // Declearign a Tuple for Fruit
    let fruit = ("Mango", 10, 250.5); // fruit name, weight, price
    //fruit.0 = "orange";
    println!("{}",fruit.0); // accessing tuple elements using indexing
    println!("{}",fruit.1);
    println!("{}",fruit.2);

    println!("______________________________");
    println!("fruit = ");
    println!("{:#?}",fruit);  // displaying whole tuple using format specifier :#?
    println!("{:p}",&fruit); // displaying 'address' of tuple 'fruit'
    println!("______________________________");
    

    println!("Destructuring the Tuple");

    let (name, weight, price) = fruit;

    println!("{}",name);
    println!("{}",weight);
    println!("{}",price);

    // array of cricket teams and year of winning world cups

    let teams = ["Pakistan", "Australia", "England", "Srilanka", "Endia"]; //cricket teams
    let win_cup = [1992, 1996, 2000, 2004, 2008]; // world cup winning years

    println!("{:#?}", teams); // displaying whole array
    println!("{:#?}", win_cup);
    
    //1st
    println!("crickt team:");
    println!("{}", teams[0]);
    println!("year:");
    println!("{}", win_cup[0]);
    println!("________________________");

    //2nd
    println!("crickt team:");
    println!("{}", teams[1]);
    println!("year:");
    println!("{}", win_cup[1]);
    println!("________________________");

    //3rd
    println!("crickt team:");
    println!("{}", teams[2]);
    println!("year:");
    println!("{}", win_cup[2]);
    println!("________________________");

    //4th
    println!("crickt team:");
    println!("{}", teams[3]);
    println!("year:");
    println!("{}", win_cup[3]);
    println!("________________________");


    //5th
    println!("crickt team:");
    println!("{}", teams[4]);
    println!("year:");
    println!("{}", win_cup[4]);
    println!("________________________");

   /* println!("crickt team:");
    println!("{}", teams[6]);  // error: index out of bounds: the len is 5 but index is 10
    println!("year:");
    println!("{}", win_cup[10]);
    println!("________________________"); */
}
