fn main() {
    let x = 3;
    if x == 5 {
    println!("x = {}", x); 
    }
    else {
        println!("x is not 3");
    }

     // Passing mutable referances

     let mut s = String::from("Hello");
     // s.push_str("! World");  //...... This will work
      let d = &s; // Declearing Immutable Referance
   //  { 
         let e = &s;
          //let e = &mut s; // mutable and imutable referance ,,, it will not work
                          // Data race ... at least 1 of the pointers being used to write the data
       
        println!("{} , {}, {}", d, e, s);
     //}
       //   {
              let f = &mut s;
             // let g = &mut s; second mutable referance will not work either
            // let h = &s; immutable borrow will not work after mutable borrow
         f.push_str("! World"); // This is also working
          println!("{}", f);
         // }

         let g = &mut s;
         //println!("{} , {}", f,g);//  data race ... 2 or ,ore pointers accessing the same data at same time
         println!("{}", g);
}
