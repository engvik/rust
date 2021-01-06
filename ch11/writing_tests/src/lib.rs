#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    /*
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    */

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger._can_hold(&smaller));
        // assert!(larger._can_hold_with_bug(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller._can_hold(&larger));
        // assert!(!smaller._can_hold_with_bug(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        // assert_eq!(4, add_two_with_bug(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));

        // let result = greeting_with_bug("Carol");
        // assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_with_message() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeing did not contain name, value was `{}`",
            result
        );

        /*
        let result = greeting_with_bug("Carol");
        assert!(
            result.contains("Carol"),
            "Greeing did not contain name, value was `{}`",
            result
        );
        */
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    /*
    #[test]
    #[should_panic]
    fn greater_than_100_with_bug() {
        Guess::new_with_bug(200);
    }
    */

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_expected() {
        Guess::new_with_multiple_panics(200);
    }

    /*
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_expected_with_bug() {
        Guess::new_with_multiple_panics_with_bug(200);
    }
    */

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal to four"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn _can_hold_with_bug(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_two_with_bug(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn greeting_with_bug(_name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { _value: value }
    }

    pub fn new_with_bug(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { _value: value }
    }

    pub fn new_with_multiple_panics(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { _value: value }
    }

    pub fn new_with_multiple_panics_with_bug(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { _value: value }
    }
}
