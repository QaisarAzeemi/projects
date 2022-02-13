fn main() {
   let mut x = 1;
   loop {                                                      // Syntex
                                                               // loop {
       if x < 5                                                // increment/ decrement
        {                                                      // if slse
           println!("You entered {}", x);                      // break }
    
           x = x + 1;

        }

     else 
        { 
           println!("value equals and exceeding {}", x);
           break;
        }
    }

    println!("\n While Loop starts");
// .........................................................................................
    while x < 10                                                 // Syntex
                                                               // while condition
    {                                                          // { increment / decrement}
         if x < 10
         {
            println!("You entered {}", x);
    
             x = x + 1;
         }
                                      
    }
    println!("value equals and exceeding {}", x);
// ........................................................................................

 println!("\n For Loop starts");

 // syntax 
 // for variable in conditions

 //for number in 1..10 // adding paranthesis in extra
 for number in 1..101//.iter()  // reverse counting function ... iter() is use to iterate the arrays while rev() is used for reverse counting
  //for number in (x<20)   // 'bool' is not an iterator
  {
     println!("You entered {}", number);
    // x = x + 1;
  }


  let f = "Stack"; // string litral
  let g = f; // premitive types stored on stack have copy traits
  println!("f = {} and g = {}",f,g);

  let h = String::from("Heap"); // string type
  //let i = h;//.clone(); // heap data dont have copy trait untill and unless we deeply copy it using clone() function
  println!("h = {}",h);
}
