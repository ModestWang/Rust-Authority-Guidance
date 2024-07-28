mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    // 注意：公共结构体的默认字段都是私有的
    // 如需公有字段，需要显式声明
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 注意：公共枚举的所有成员都是公有的
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apples"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("blueberries"); // error: private
}
