extern crate core;
#[derive(Debug)]
pub struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    pub fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess{
        if value < 1 || value > 100{
            panic!("aeyo not in range");
        }
        Guess{value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explore() ->Result<(), String> {
        if 2 + 2 == 4{
            Ok(())
        }else{
            Err(String::from("two plus two not four"))
        }
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{ width: 8, height: 7 };
        let smaller = Rectangle{ width: 4, height: 1 };

        assert!(larger.can_hold(&smaller));
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

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected = "aeyo not in range")]
    fn greater_than_max(){
        Guess::new(200);
    }
}
