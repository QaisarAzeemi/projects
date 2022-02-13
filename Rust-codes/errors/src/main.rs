#![allow(unused_variables)]
use std::fs;
use std::io::{self, Read, ErrorKind};
// crate::Module::Struct
fn main() {
 //   println!("Hello, world!");
   // panic!("crash ans burn"); // panicked in our file

   // panic at rust source code...

   //let x = vec![0,1,2,3,4];
   //x[7]; // index out of bound... vector is decleared in src\libcore\slice\mod.rs
   // at line # 2715 and character # 10 (2715:10)

   // Recoverable errors with results .........................................................
   /*
   let f = File::open("Hello.txt"); // Err(IO::Error) struct
   let f = match f {
     Ok(file) => file,
     Err(error) => match error.kind(){ //kind() is a method of struct Error that contains an enum ErrorKind
      ErrorKind::NotFound => match File::create("Hello.txt"){
      Ok(f) => f,
      Err(fc) =>  panic!("error file creation"),
      }, 
       _ => {panic!("No Permission");
      },
     },
   };
 */

 println!("{:#?}", read_username_from_file()); 
}

//... Propagating errors..............................................................................
/*
fn read_username_from_file()-> Result<String, io::Error>{
let f = File::open("Hello.txt");

let mut f = match f {
 Ok(file) => file,
 Err(e) => return Err(e),
};

let mut s = String::new();
match f.read_to_string(&mut s){ // this method will read the contents of file
  Ok(_) => Ok(s),
  Err(e) => Err(e),
}
}
*/
//........... Shortcut for propagating error.....................................................

fn read_username_from_file() -> Result<String, io::Error>{

  let mut s = String::new();
 // File::open("Hello.txt")?.read_to_string(&mut s)?; // ? operator will do just like result enum
 // Ok(s)

 // or even shorter ...... using

 fs::read_to_string("Hello.txt") // for this we have to eleminate file form use
}