mod front_of_house;
mod front_of_house2;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house2::hosting2;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
}
