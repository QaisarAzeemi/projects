fn main() {
    let s = String::new();
    let mut s1 = String::from("String, Common collections");
    println!("{}", s1);
    println!("{}", s);
// Changing string literal to string type using to_string() method...........................
    let s2 = "Pakisatan Zinda Baad";
    let data = s2.to_string();
    println!("{}", data);

    let s3 = "ki haal ay".to_string();
    println!("{}",s3);

    println!("Theek Thaak ho??");//.to_string());

    //  Displaying characters of other languages.....UTF-8.......................................
    let s4 = String::from(" علیکم اسلام");
    println!("{}", s4);

    // Updating the string.........................................................................

    s1.push_str(", Complete Guid"); // method to push the string
    println!("{}", s1);
    
    s1.push('e'); // method to push a single character
    println!("{}", s1);
    
    // String Concatenation.........................................................................
   let s5 = String::from("tic");
   let s6 = String::from("tac");
   let s7 = String::from("toe");

   //String concatenation appends the string on the right to the string on the left and may require
   //reallocation. This requires ownership of the string on the left.
   let concat = s5 + "-" + &s6 + "-"+ &s7;
   println!("{}", concat);
   //println!("{}",s5); can't be used as it is already borrowed in concat
   let s5 = String::from("tic");

   // format!() macro is just like println!() but doesn't display. it rather returns a concatinated string.
   // it doesn't takes the ownership also
   let s8 = format!("{} - {} - {}", s5, s6, s7);
   println!("{}", s8);

   let s8 = format!("{} & {} & {}", s5, s6, s7);
   println!("{}", s8);
   println!("{}",s5);
   
   // Indexing in strings..........................................................................
   // indexing of String type is not possible ... While
   // indexing of string literal is possible

   let s9 = "Pakistan";
   let ind = &s9[0..8];
   println!("{}", ind);

   let s10 = "巴基斯坦"; // each chinese character is taking 3 bytes
   let indc = &s10[0..12];
   println!("{}", indc);

   let s11 = "Пакистан"; // each russian character is taking 2 bytes
   let indr = &s11[0..12];
   println!("{}", indr);

   let s12 = "باكستان"; // each arabic character is taking 2 bytes
   let inda = &s12[0..12];
   println!("{}", inda);

   let s13 = "पाकिस्तान"; // each hindi character is taking 3 bytes
   let indh = &s13[0..27];
   println!("{}", indh);

   // Itteration in strings.............................................................................
    
   for c in "पाकिस्तान" . chars(){ // to display all characters in a string separately
       println!("{}",c);
   }

   for c in "पाकिस्तान" . bytes(){ // to display all bytes UTF-8 code that a string contains
    println!("{}",c);
}

}
