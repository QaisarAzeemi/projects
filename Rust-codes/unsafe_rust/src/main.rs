fn main() {
    let mut num = 26;
    println!("<<< value of new number is : {}", num);
    println!("<<< Referance of new number is : {}", &num);
    println!("<<< address of new number is : {:p}", &num);
    println!("<<< binary of new number is : {:b}", &num);
    println!("<<< Hexa of new number is : {:x}", &num);


// Declearing a Raw Pointer


   let ref1 = &num as *const i32; // Raw pointer
   println!("<<< address of pointer to number is : {:p}", ref1);

   unsafe{ // unsafe rust is necessary to dereferance a pointer
   println!("<<< Pointer of new number is : {:?}", *ref1); // dereferencing a raw pointer 
   }

   let ref2 = &mut num as *mut i32;
   println!("Reference 2 of the pointetr is : {:p}", ref2);
   unsafe{ // unsafe rust is necessary to dereferance a pointer
   println!("value of reference pointer 2 is : {:?}", *ref2);
   }
   unsafe{ // unsafe rust is necessary to dereferance a pointer
   *ref2 = 79;
   println!("changed value of 2nd pointer is : {:?}", num);
   }
}
