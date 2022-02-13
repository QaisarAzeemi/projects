
mod lib;
//use enums::ep;  // using library crate (this syntex is also true)
//ep(); it will not run because we haven't call it in main()
/*
#[derive(Debug)]
enum Student {      // enum definition
    Online(String),
    Onsite(String),
}

fn main() {
    ep();
    let student1 = Student::Online(String::from("Qaisar"));       //  enum instance
    let student2 = Student::Onsite(String::from("Azeemi"));
    println!("{:?}", student1);
    //route (Student::Online);
    route (student2);
}

fn route(std_name: Student){
    println!("{:?}", std_name);
}
*/
//_________________Accessing enums via structs_____________________________________________________________________________
//_________________________________________________________________________________________________________________________
/*
#[derive(Debug)]
enum IpAdderKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAdder{
    kind: IpAdderKind,
    value: String,
}
fn main () {
       ep();
    let address1 = IpAdder {
        kind: IpAdderKind::V4,
        value: String::from("127.0.0.1"),
    };

    let address2 = IpAdder {
        kind: IpAdderKind::V6,
        value: String::from("::1"),
    };

    println!("{:#?}", address1);
    println!("{:#?}", address2);
}
*/
//_________________Accessing enums directly_____________________________________________________________________________
//_________________________________________________________________________________________________________________________
/* 
#[derive(Debug)]
enum IpAdder {
    V4(String),
    V6(String),
}


fn main () {
     ep();
    let address1 = IpAdder::V4(String::from("127.0.0.1"));
    let address2 = IpAdder::V6(String::from("::1"));

    println!("{:?}", address1);
    println!("{:?}", address2);
}
*/
//_________________Accessing enums directly_____________________________________________________________________________
//______________________________________________________________________________________________________________________
/*  Predefined standard library enum Option doesn't need to be difined

#[derive(Debug)] // Defining Option enum
enum Option<T>{
    Some(T),
    None
}  */

fn out_of(o: &Message){ // function decleared out of enumm implementation block
    println!("{:?}", o); // these functions can be decleared anywhere out of main funtion
}

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move{x:i32, y: i32}, // annonymus struct
    Color(u32, u32, u32),
}

// Implementation blocks for enums are just like structs
impl Message{ // Implementation block to pass enum instances and disply
    fn call(&self, recv: &Message){ // defining a method , receiving 2 instances
        println!("it is enum Message Implementation Block.........................");
        println!("{:?}", self);
        println!("{:?}", recv);
    }

    fn in_to(o: &Message){ // associated dunction
        println!("{:?}", o);
    }
    
}
 

fn main (){
   lib::ep();
    // ep(); 
    // declearing instances of enum Message
    let msg1 = Message::Quit;

    let msg2 = Message::Write(String::from("Message 2"));

    let mut msg3 = Message::Move{x:5 , y:-7}; // making this instance mutable for changes
    //Message::Move.x = 8;
    let mut msg4 = Message::Color(0,0,0);
    //Message::Color.0 = 255;

    let out_func = Message::Write(String::from("This value is passed in functon out of implemention block..........!"));
    out_of(&out_func);
    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("{:?}", msg3);
    println!("{:?}", msg4);

    msg2.call(&msg3);

    let in_func = Message::Write(String::from("This value is passed in functon into the implemention block...!"));
    Message::in_to(&in_func); // calling an associated function into the enum implementation block
 //.......... for Option enum...............................................................................................
 //.........................................................................................................................

  /*let some_number = Option::Some(5);
  let some_string = Option::Some(String::from("Its the string for Option enum................!!!"));
  let no_value : Option<i32>  = Option::None;*/
  
  // Since Option is defined in Rust Standard Library so we don't need to access some using Option:: ... we can directly access the Some() ... Rust knows .... 
  let some_number = Some(5);
  let some_string = Some(String::from("Its the string for Option enum................!!!"));
  let string_litral = Some("'String Literal'");
  let no_value : Option<i32>  = None;
  
  println!("{:?}", some_number);
  
  println!("{:?}", some_string);

  println!("{:?}", string_litral);

  println!("{:?}", no_value);

}

 