// Restaurant example .... page # 126 of new book.... 
// Restaurants have 2 main departments ... 1) Front of house and 2) Back of house
// Front of house contians customer seating and hosting while back of house contain Cooking, management, cleaning etc

#![allow(dead_code)]
#![allow(unused_variables)]

pub mod customer_experiance{
  pub mod front_of_house { // Parent module
    pub mod hosting { // Child module
        pub fn adding_waitlist () {println!("Name added to waitlist");}

        fn provide_seating() {println!("Seating provided");}
    }

    mod serving { // Child module
        fn taking_order() {println!("order taken.....!");}

        fn serving_order() {println!("order served.....:)");}

        fn taking_payments() {println!("Payment taken.....:)");}

    }
}
}
pub mod dining{
 pub fn eat_at_restaurant(){
 
    // Absolute path
   // crate::customer_experiance::front_of_house::hosting::adding_waitlist();

    // Relative path
    super::customer_experiance::front_of_house::hosting::adding_waitlist();

    let mut meal = super::back_of_house::Breakfast::summer("Rye");
    println!("{:?}", meal);
   //meal.toast = String::from{"Wheat"};
    
    let start = super::back_of_house::Appetizer::soup;
    println!("{:?}", start);

}
}

mod back_of_house {

    #[derive(Debug)]
    pub enum Appetizer {
        soup,
        salad
    }

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast{
        pub fn summer(toast: &str)-> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peach")
            }
        }
    }
}