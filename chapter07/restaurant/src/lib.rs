// nested paths
use std::{cmp::Ordering, io};
use std::io::{self, Write};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod front_of_house{
    // adding pub keyword because inner abstraction is private by default
    pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn server_order(){}
        fn take_payment(){}
    }
}

pub fn eat_at_restaurant(){
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::Summer(
        "Rye"
    );
    meal.toast = String::from("wheat");
    println!("i'd like {} toast pelase", meal.toast);
    back_of_house::Appetizer::Soup;
}

fn delivery_order(){}

mod back_of_house{
    //we just te enum as pub, your attribute already are
    //public by default
    pub enum Appetizer{
        Soup,
        Salad,
    }

    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn Summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrerct_order(){
        cook_order();
        super::delivery_order();
    }

    fn cook_order(){}
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
