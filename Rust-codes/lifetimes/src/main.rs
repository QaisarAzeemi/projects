#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[derive(Debug)]
struct importatExcerpt <'a> {
    part : &'a str,
}

impl <'a, 'b> importatExcerpt <'a> {
    fn new(&self) -> i32 {
        3
    }

    fn announce_return_part(&'a self, announce:&'b str) -> &'a str {
        println!("{}",announce);
        self.part
    }
}

/*fn main() {
   let r;
    
   {
       let x =5;
       let r = &x;
   } // x.drop();
       println!("{:p}",r); 
   // this code will give an error due to dangling referance 
} */

// writing a logic to compare and find the longest string 
fn main(){
    let s1= "abcd";
    let s2= "xyz.......";
    let result;
    println!("\n{}",longest(s1,s2));

    let s3 = String::from("Hello World!");
    {
        let s4 = String::from("Hello Pakistan!");
        result = longest(&s3.as_str(), &s4.as_str());
        println!("{}",result);
    }
    //println!("{}",result); // s4 doesn't live long enough
    let result2 = println!("{}",test(&s3));
    //let a = 1354;
    //let b = 1548.225;
   // let result3 = println!("{}",test2(&a,&b));
    let novel = String::from("its raining outside. No, you are laying. its blazing sun.");
    let firstSentense = novel.split('q').next().expect("Couldn't find a '.'");
    let i = importatExcerpt {
        part : firstSentense,
    };
    println!{"\n{:?}\n",i};

   println!("{}",i.new());
   println!("{}",i.announce_return_part("Hello World!"));
}

fn longest <'a>(x:&'a str, y:&'a str)->&'a str{
    if x.len()>y.len() {
        println!("{}",x.len());
        x
    }
    else{
        println!("{}",y.len());
        y
    }
}

// fn longest <'a>(x:&'a str, y:&str)->&'a str{
//     x
// }

fn test(x: &String) -> &String{
    x
}

/*fn test2 <'a,'b,'c>(x:&'a u32,y:&'b f64) -> &'c f64 {
    &x as f64 + y
}*/
// <Hello>
// hellow Pakistan
// sham is my non_camel_case_types
// qaisar azeemi