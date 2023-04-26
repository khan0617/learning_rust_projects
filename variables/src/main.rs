use std::io;

fn get_y() -> i32 {
    {
        let x = 10;
        x + 1
    }
}

fn five() -> i32 {
    5
}

fn main() {
    // for and while loops
    // let mut number: i32 = 3;
    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    // using an inclusive range w/ for loop
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // let a: [i32; 5] = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("The value is: {element}");
    // }


    // ranges and rev
    // for number in (1..=4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF");

    // function calls and ternary operator
    // println!("value of y is {}", get_y());
    // println!("Five: {}", five());
    // let y = get_y();
    // let five = five();
    // let maximum = if y > five { y } else { five };
    // println!("Maximum is: {maximum}");

    // loops as expressions
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 10;
    //     }
    // };
    // println!("The result is {result}");

    // variable practice, arrays and tuples
    // let tup: (i32, f64, &str) = (32, 6.4, "Hello!");
    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // reading from std input
    // let mut index = String::new();
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
}