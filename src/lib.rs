// Annotation tells Rust not to compile the code in this module
// unless the command 'cargo test' is run
// Note: Common convention is to include a test module in each
// src code file
#[cfg(test)]
mod tests {
    // Used to bring the outer module inner for testing
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // above test rewritten with the Result<T,E> type
    // return an Err instead of a 'panic'
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /* commented out so that the integration
    test of the same name can run
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }*/

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            // extending the assert with a useful output message on failure
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[ignore] // Ignored to allow all valid tests to pass
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        // following test rules we first define
        // dummy data and then we test the function
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        // following test rules we first define
        // dummy data and then we test the function
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        // ! to define the fact we want a false value
        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // Indicates test should panic and thus PASS if so
    // 'expected' section ensure that the right panic message is returned
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value)
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value)
    }
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Example of a function which returns bool
// so we can display the assert! functionality
impl Rectangle {
    #[cfg(test)] // added to suppress the warning 'associated function is never used'
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

// implementing the Guess example for testing
// example of checking for panic with 'should_panic'
pub struct Guess {
    #[allow(dead_code)] // Added to suppress warning 'field is never read'
    value: i32,
}

impl Guess {
    // function refined to give more explicit panic messages for debugging
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}",
                   value
            );
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}",
                   value
            );
        }

        Guess { value }
    }
}

