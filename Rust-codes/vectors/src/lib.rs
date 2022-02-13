// pushing and popping values in a vector

pub fn see(){
let mut _v3 : Vec<i32> = Vec::new();
_v3.push(45); // push(&element) Method is used to add value to last index of a vector
_v3.push(48);
_v3.push(77);
_v3.push(102);
_v3.push(199);
println!("{:?}", _v3);
_v3.pop(); // pop() method is used to remove the value from last index of the vector
println!("{:?}", _v3);
}   