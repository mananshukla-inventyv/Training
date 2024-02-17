// use restaurant::front_of_house::hosting;
// use restaurant::front_of_house::serving::ser;
// use restaurant::back_of_house::cooking::cook;

mod front_of_house;
mod back_of_house;

// mod lib;
// use ::front_of_house;

use front_of_house::serving::ser;
use back_of_house::cooking::cook;




fn main(){
    // println!("{:?}" , );
    cook();
    ser();
    front_of_house::hosting::hosting(); //using mod
    front_of_house::serving::ser();
    back_of_house::cooking::cook();
    // front_of_house::hosti;ng::hosting()
}