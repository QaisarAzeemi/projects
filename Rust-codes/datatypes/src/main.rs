fn main() {
    // Rust has 2 Premitive data types (i) Scalar (ii) Compound
    // scaler data types ... includes... integer, float, character and Boolean
    // scalar data types can contain a single value while compound data types can contain multiple values
    // Compound data types are (i) Tuples (ii)Arrays
    // no need to specify type explecitly in both data types .. rust is statically typed.. it knows types

    const PI : f64 = -3.14159;   println!("{}", 2 as f64 * PI); // multiply it either 2.0 or use keyword "as" for type conversion
    const g : f64 = 9.8;        println!("{}", PI * g);

    let age = 35;            println!("{}", age);
    let percentage = 82.1;   println!("{}", percentage); // we can also specify : f 32
    let grade = 'A';         println!("{}", grade);
    let pass = true;         println!("{}", pass);      // // we can also specify : bool
    let neg_num : i32= -52;  println!("{}", neg_num); // signed integer , negative number

    // Number systems ,,, how to declear?

    let decimal = 14;        println!("{}", decimal);
    let hexa_decimal = 0xff; println!("{}", hexa_decimal); // 0x is used to display hex value as decimal ... if we use {:b} we can display it as binary
    let octal = 0o77;        println!("{}", octal);
    let binary = 786;         println!("{:b}", binary);  // :b format is use to display as binary 
    let c = b'A';            println!("{}", c); //for byte only (Note: check the other format)

}


