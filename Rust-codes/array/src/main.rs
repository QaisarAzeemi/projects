fn main() {
    
    let x = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let y = [10 ;32]; // arrays only have std trait implementations for lengths 0..=32
    let z = ['a' , 'b', 'c'];
    let s = ["Muhammad", "Qaisar", "Azeem"];

    println!("{:#?}", x);
    println!("{:?}", y);
    println!("{:#?}", z);
    println!("{:#?}", s);
    println!("{}", x[3]);

    // array of cricket teams and year of winning world cups

    let teams = ["Pakistan", "Australia", "England", "Srilanka", "Endia"]; //cricket teams
    let win_cup = [1992, 1996, 2000, 2004, 2008]; // world cup winning years

    println!("{:#?}", teams);
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

    println!{"Team: {}; win year: {}", teams[3],win_cup[4]};

}
