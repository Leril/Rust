struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let rect1 = Rectangle{
    //     width: 30,
    //     height: 50,
    // };
    // println!("The area of the rectangle is: {}", rect1.area());
    // println!("Rect1 is {:?}", rect1);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(12);
}

fn structs(){
    let mut user1 = User{
        active: true,
        username: String::from("user123"),
        email: String::from("email@some.com"),
        sign_in_count: 1
    };

    // let user2 = User{
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("newmail@some.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User{
        email: String::from("newmail@some.com"),
        ..user1
    };

    user1.username = String::from("anothermail@some.com");
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
