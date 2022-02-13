//...................Finding Area of a Ractangle............................................

#[derive(Debug)]
struct Ractangle {
    width: u32,
    height: u32
} 
 impl Ractangle {
    fn area (&self) -> u32{ // Defining method area for struct Rectangle
        self.width * self.height
      
    }

    fn square(size : u32) -> Ractangle{ // Associated function
        Ractangle {
            width: size,
            height: size
        }
    }
}
// ................ passing 2 arguments........................................................................
//.............................................................................................................

/*fn main() {
    let width1 = 50;
    let height1 = 10;
    let result = area( width1 , height1);
    println!("Area of Rectangle is {}", result);
}

fn area(width:u32, height:u32) -> u32
{
    width * height
}*/

//..........Passing 1 parameter using Tuple... 
//..........Making code more organised........

/*fn main(){
    let ract = (50,10); // declearing a tuple
    let result = area(ract);
    println!("Area of Rectangle is {}", result);
    println!("Dimentions of Rectangle are {:?}", ract);

}

fn area(cup : (u32, u32)) -> u32 // the value of rect will be copied in cup.. it is just like assignment.. *Premitive data types have copy trait
{
    cup.0 * cup.1
}*/

//.................. using structure (user defined data type)............................................................
//.......................................................................................................................
fn main(){
 let rect = Ractangle {
     width: 50,
     height: 30
 };

 let rect2 = Ractangle {
    width: 500,
    height: 420
};

 let mut calc = rect.area();
 println!("Area of Rectangle = {} square meters" , calc);
 println!("Dimentions of Rectangle = {:#?} in meters" , rect);
 println!("_________________________________________________");
 println!("_________________________________________________");
 calc = rect2.area();
 println!("Area of Rectangle = {} square meters" , calc);
 println!("Dimentions of Rectangle = {:#?} in meters" , rect2);
 println!("_________________________________________________");
 println!("_________________________________________________");
 println!("square of ractangle is {:#?}", Ractangle::square(8)); // calling associated function using namespacing :: operator
}

//fn area (sheet : &Ractangle) -> u32{ // Rectangle is user defined data type
  //  sheet.width * sheet.height
//}



