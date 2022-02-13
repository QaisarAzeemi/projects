#![allow(dead_code)]
#![allow(unused_variables)]
const PI : f64 = 3.14159;   // Declearing Global constant with data type float (f64) 64bits
 fn main() {
    let temprature = 44; // Declearing variable (Immutable bydefault)
   // temprature = 12;
   let mut pressure = 120;  // Declearing mutable variable pressure
   pressure = 778;          // now we can assign a new value to same variable without writng let at start
   println!("Temprature"); // Displaying String
    println!("{}", temprature);
    println!("{}", pressure);
    println!("{}", PI);

    let x = String::from("Hello!"); //string type to save data into heap, its pointer will be saved into stack
    //let y = x;  //----> only heap pointer will be copied... heap data will not be copied
    let y = x.clone(); // deep copy the heap data
    println!("{}", y);

    let a = 123;
    let b = a; // stack only data copy
    println!("{} copied into {}", a , b);
 
    let c = &a;
    println!("{}", c);
    println!("Address of b is : {:p}", c);

    // Passing mutable referances

    let mut s = String::from("Hello");
   // s.push_str("! World");  //...... This will work
    let d = &s; // Declearing Immutable Referance
    let e = &s;
     
    //println!("{} , {}, {}", d, e, s);
  
        let f = &mut s;
       f.push_str("! World"); // This is also working
        println!("f = {}", f);


  //________________________________________________________________________________________________________
  //________________________________________________________________________________________________________      
        let mut num = 32;
        println!("num = {}", num);

        let r1 = &mut num as *mut i32; // This *mut i32 is raw pointer decleration
        println!("pointer to num is = {:?}", r1);

        // NOTE: formatters :p is to display address of memory location
        //                  :b is to display binary value
        //                  :x is to display hexadecimal value
        // now i want to display value of r1 using raw pointer

        unsafe { // dereferencing of raw pointer is not allowed in Rust so we have to use unsafe block
        println!("Value of r1 using raw pointr is {}", *r1); // This is called Dereferancing of raw pointer
        }
}



