//page # 188 of new book
#[derive(Debug)]
struct Point <T, U> {
    x: T,
    y: U,
}

impl <T, U> Point <T, U>  {
    fn x(&self) -> &T {
        &self.x     // we can't do binary operations on Generic types
        
    }

    fn y(&self) -> &Point <T, U> {
        &self
    }

    fn mixup <V,W> (self, other : Point<V,W>) -> Point <U,V> { // function signature
        Point {      // function body
            x: self.y, // this is self.key
            y: other.x,
        }
    }
}

impl Point <f64, f64> {      // make this block for binay operations
    fn distance(&self) -> f64 {
        (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
        let mut largest = &numbers[0];
            for number in &numbers { // avoiding transfer of ownership
                if number > largest {  
                    largest = number;
                }
            }
    println!("\nlargest number is = {}\n" , largest);

    let tests : Vec<u64> = vec![25, 50, 450, 1000, 45878];
    let z = &tests[0];
    for x in &tests {
    println!{" x = {}" ,x};
    }
    let y = [1, 25, 14, 36, 68];
    for x in y.iter(){
        println!("x = {}", x);
    }
    println!("y = {:?}", y);// */ 

    println!("_____________________________________________________________");

    let integer1 = Point { x : 10, y : 20};
    let float1   = Point { x : 63.15, y : 25.501};
    let int_float = Point {x : 14, y : 693.47};
    let info = Point { x: "VS code" , y : 'c'};

    println!("\n{:?}",integer1);
    println!("{:?}\n", float1);
    println!("{:?}\n", int_float);

    println!("_____________________________________________________________");

    println!("{}", integer1.x());
    println!("{:?}", integer1.y());

    println!("______________________Distance_______________________________");
    println!("{}\n", float1.distance());

    println!("______________________Mixup_______________________________________");
    println!("{:?}", int_float.mixup(info));

}
    