struct AlwaysEqual;

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// struct with explicit lifetime parameter
struct User <'a>{
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method area instead of standalone function
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Function (not a method, does not have self as 1st param)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// build struct with explicit lifetime
fn build_user<'a> (email: &'a str, username: &'a str) -> User<'a> {
    User {
        active: true,
        email: email,
        username: username,
        sign_in_count: 1
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let user1 = build_user("a@gmail.com", "some_username");

    // we can create a new object using user1's fields
    // however, this means that the username will transfer ownership to user2 here.
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("user2@gmail.com"),
    //     sign_in_count: user1.sign_in_count
    // };

    // we can write this faster using .. syntax:
    // let user2 = User {
    //     email: String::from("user2@gmail.com"),
    //     ..user1
    // };

    // println!("user1: {:?}", user1);

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    // unit-like structs, like (), have no fields.
    // let subject = AlwaysEqual;

    // dbg! macro and functions using structs
    // dbg! can go around any expression basically
    let scale = 2;
    let rect1 = Rectangle {
        // width: dbg!(30 * scale),
        width: 30,
        height: 50,
    };
    // dbg!(&rect1);
    println!("rect1 is {:#?}", rect1); // pretty print w/ {:#?}
    println!("Area of rect1: {}", rect1.area());

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

    // use Rectangle's Associated Function like this:
    let square = Rectangle::square(32);
    println!("Square: {:#?}", square);

}
