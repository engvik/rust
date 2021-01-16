// use publishing::kinds::PrimaryColor;
// use publishing::utils::mix;

use publishing::mix;
use publishing::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
