struct User {
    username: String,
    sign_in_count: u64,
}

struct Color(i8, i8, i8);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        username: String::from("someone@example.com"),
        sign_in_count: 26,
    };

    println!("Hello, world: {}", user1.username);

    let mut _user1 = user1;

    _user1.username = String::from("another@email.com");

    let user2 = build_user(String::from("bob"));

    println!("hi {} for the {} time", user2.username, user2.sign_in_count);

    let user3 = User {
        username: String::from("third@email.biz"),
        ..user2
    };

    println!("User 3 email: {} count: {}", user3.username, user3.sign_in_count);

    let c = Color(1,2,3);
    let Color(r, _, _) = c;
    println!("r:{} g:{} b:{}", r, c.1, c.2);

    let r = Rectangle { width: 5, height: 10 };

    println!("Rectangle debug format: {:?}", r);
    println!("Rectangle pretty debug format: {:#?}", r);
    println!("Area: {}", area(&r));
    println!("Area method: {}", r.area());

    let r1 = Rectangle { width: 30, height: 50 };
    let r2 = Rectangle { width: 10, height: 40 };
    let r3 = Rectangle { width: 60, height: 45 };

    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}", r1.can_hold(&r3));

    let r = Rectangle::square(15);
    println!("Square 15: {:?}", r);

    let _can_hold = Rectangle::can_hold(&r1, &r2);
}

fn build_user(username: String) -> User {
    User {
        username,
        sign_in_count: 1,
    }
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
