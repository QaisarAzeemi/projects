//https://github.com/Dhghomon/easy_rust#type-inference
fn main() {
    println!("Hello, world!");
    let a : u8 = 45;
    let b = 55;
    println!("Sum of {} and {} is : {}", a, b , a+b);
    let c = &b;
    println!("The address of c is : {:p}",c);
    println!("The value of c is : {}", *c);
    println!("The number form another function is : {}", number());
    println!("\n_________________________________________________________________________");
    println!("The smallest i8 is {} and the biggest i8 is {}.", i8::MIN, i8::MAX); // hint: printing std::i8::MIN means "print MIN inside of the i8 section in the standard library"
    println!("The smallest u8 is {} and the biggest u8 is {}.", u8::MIN, u8::MAX);
    println!("The smallest i16 is {} and the biggest i16 is {}.", i16::MIN, i16::MAX);
    println!("The smallest u16 is {} and the biggest u16 is {}.", u16::MIN, u16::MAX);
    println!("The smallest i32 is {} and the biggest i32 is {}.", i32::MIN, i32::MAX);
    println!("The smallest u32 is {} and the biggest u32 is {}.", u32::MIN, u32::MAX);
    println!("The smallest i64 is {} and the biggest i64 is {}.", i64::MIN, i64::MAX);
    println!("The smallest u64 is {} and the biggest u64 is {}.", u64::MIN, u64::MAX);
    println!("The smallest i128 is {} and the biggest i128 is {}.", i128::MIN, i128::MAX);
    println!("The smallest u128 is {} and the biggest u128 is {}.", u128::MIN, u128::MAX);

    println!("\n____________________escape characters___________________________________________");
    
    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."); // We used \ five times here
    println!(r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#);
    println!("{:?}", b"This will look like numbers");
    println!("#\\");
    println!(r#"#\"#);
    let many_hashtags = r####""you dont need to write ###. you just need to write #""####;
    println!("{}",many_hashtags);
    let title = "Qaisar's Programs";
    println!("{:-^31}",title);//padding; the : and has a ^. < means padding with the character on the left,
                                //> means on the right, and ^ means in the middle
    let bar = "|";
    println!("{: <15} {: >15}",bar, bar);
    let city1 = "Peshawar";
    let city2 ="Karachi";
    println!("{city1: <15} {city2: >15}");

    let mut country = String::from("Pakistan");
    let updatedstring = getCountry(&mut country);
    println!("\n{}",updatedstring);

    println!("\n____________________Arrays and vectors and Tuples_________________________");
    let arr = [1,2,3,4,5,6,7,8,9,10];
    println!("{:?}",arr);
    let mut vec1 = Vec::new();
    vec1.push(8);
    vec1.push(71);
    println!("{:?}",vec1);
    let tup1 = ("This is a Tuple ", 8, vec!['b','c','d'], 7.6, 'e');
    println!( "Inside th Tuple is : 
    First Item: {:?} 
    Second Item: {:?}
    Third Item: {:?}
    Fourth Item: {:?}
    Fifth Item: {:?}", 
    tup1.0,
    tup1.1,tup1.2,tup1.3,tup1.4);
    println!("\n____________________Match statemant control flow_____________________________");
    
    let my_number = 5;
    match my_number {
        0 => println!("number is 0"),
        1 => println!("number is 1"),
        2 => println!("number is 2"),
        3 => println!("number is 3"),
        4 => println!("number is 4"),
        5 => println!("number is 5"),
        _ => println!("Number is Irrelavant....."),
    };
    let double = match my_number {
        0 => 0,
        1 => 1,
        2 => 4,
        3 => 6,
        4 => 8,
        5 => 10,
        _ => 111111
    };
    println!("The double of {} is {}",my_number,double);
}

fn number() -> f64 {
    67.
}

fn getCountry(country2 : &mut String) -> &mut String{
       println!("{}", country2);
       country2.push_str(" Zinda baad");
       country2
}