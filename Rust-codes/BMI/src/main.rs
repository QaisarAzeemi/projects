use std::io;
fn main() {
    let mut BMI : f64 = 0.0;
    let mut mass = String::new(); 
    let mut height = String::new(); 
        println!("Hello, Enter your mass: {}", mass);
        
    io::stdin().read_line(&mut mass).expect("Wrong value");
    let mass : f64 = mass.trim().parse().expect("Wrong value");
   
   //______________________________________________________________________________

    println!("Now, Enter your height: {}", height);
    io::stdin().read_line(&mut height).expect("Wrong value");
    let height : f64 = height.trim().parse().expect("Wrong value");  
      
//______________________________________________________________________________
    //let mut m = mass;
    //let mut h = height;
    BMI = mass / (height * height );
    println!("_______________________________");
    println!("your BMI is: {}", BMI);


}
