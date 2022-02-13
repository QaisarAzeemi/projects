use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 10);

    map.insert(String::from("Yellow"), 50);

    println!("{:?}", map);

//.... Constracting hashmap using collect method..........by joining 2 vectors

let teams = vec![String::from("Red"), String::from("Green")]; // this vector contains keys
let init_scores = vec![22, 41]; // this vector contains values
// iterate key; zip and iterate the value, and then collect to make hashmap
let scores : HashMap<_,_> = teams.iter().zip(init_scores.iter()).collect();
println!("{:?}", scores);

// accessing values in HashMap................... using get() method
// we have to put key in get() and it will return us its value .. get() method is case sensitive.
let team_name = String::from("Blue");

let result = map.get(&team_name); // getting value of the key stored in hashmap

println!("{:?}", result);

// .... Accessing using for loop.......................................

for (key, value) in &map{
    println!("{} , {}", key, value);
}

// .... Updating a HashMap...............................................
// Overwriting the value
map.insert(String::from("Blue"), 63);
println!("{:?}", map);

// Inserting value if the key has no value...............................
// usind entry() and or_insert() methods

map.entry(String::from("Blue")).or_insert(88); // blue already exist so it doesn't update the vsalue 
map.entry(String::from("White")).or_insert(76);// white doesn't exist.. so field & value inserted
println!("{:?}", map);
}
