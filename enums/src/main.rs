// create an enum like this
enum IpAddrKind {
    V4,
    V6,
}

// this is inefficient, we can directly type our enum!
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    V4(String),
    V6(String),
}

// here's an enum with lots of different types
enum Message {
    Quit, // no data associated
    Move { x: i32, y: i32 }, // named fields, like a struct
    Write(String), // single String for data
    ChangeColor(i32, i32, i32), // tuple of three values
}

// the above enum is similar to defining the following structs:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// we can also implement some methods on enums like we did w/ structs
impl Message {
    fn call(&self) {
        // method body here...
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// example of using match with enums
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { // use brackets for multi-line blocks
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
}

// matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    // we can create instances of each variant of IpAddrKind like this
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V4(String::from("::1"));

    // we can instantiate a message m like this
    let m = Message::Write(String::from("Hello"));

    // let's look at the option enum now
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // we can't do this!
    // need to convert the Option<T> to a T before performing T operations.
    // let sum = x + y;
    // value_in_cents(Coin::Quarter(UsState::Alaska));

    // using Option<i32> with our plus_one function for matching states
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // catch all patterns using _
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    // "if let" syntax for less verbose pattern matching
    // good if we only care about enum being a certain state
    let config_max = Some(3u8);
    
    // we could do something like this:
    match config_max {
        Some(max) => println!("The max is {}", max),
        _ => (),
    }

    // this is more concise, but we lose the exhaustive checks against all
    // cases that the match state guarantees at compile time
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // we can even do "if let" statements with else branches:
    let mut count = 0;
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

}
