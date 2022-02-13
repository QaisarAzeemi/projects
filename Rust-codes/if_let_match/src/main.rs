fn main() {

  let some_value = Some (5);

    match some_value {
        Some(3) => println!("three"),
        _ => ()
    }

   let second_value = Some(7);
   if let Some(6) = second_value {
      println!("Six"); 
    }
    else {
      println!("non");
   }
    // println!("Hello, world!");
}
