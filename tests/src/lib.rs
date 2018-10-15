#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be over 1");
        } else if value > 100 {
            panic!("Guess value must be under 100");
        }

        Guess { value }
    }
}

fn _internal() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assertions() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn failure() {
        // panic!("Test fails");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was: {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "must be over")]
    fn panics_on_small() {
        Guess::new(0);
    }

    #[test]
    fn using_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("math borked"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_failing_test() {
        assert!(false, "This test only runs when cargo test -- --ignored");
    }

    #[test]
    fn can_access_private_fn() {
        assert!(_internal());
    }
}
