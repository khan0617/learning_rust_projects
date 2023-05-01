fn takes_ownership(some_string: String) {
    println!("took ownership of {some_string}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours~");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", new text from change_string!!!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// example: returning a tuple and giving ownership back 
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

fn main() {
    // let s = String::from("hello"); // s is available in this scope

    // this function takes ownership of s
    // after the function is done, drop(s) is automatically called to free the mem allocated for s.
    // s is no longer usable in the scope of main as a result
    // takes_ownership(s);
    // to overcome this, we could pass s as a reference and modify takes_ownership:
    // takes_ownsership(&s). w/ this, s is still usable in main's scope

    // try again w/ new functions:
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
        // s2 is moved into the function scope, which moves its return
        // value into s3. s2 is no longer usable at this point, but s3 is

    // since we are using a reference, calculate_length will NOT 
    // call drop() after the function completes
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);

    // modifying a string using references. s1 must be "mut"
    // in the function call, we clarify we will change s1 using "&mut s1"
    // let mut s1 = String::from("hello");
    // change_string(&mut s1);
    // println!("Modified string: {s1}");

    // we can only have one mutable reference to a variable:
    // let mut s = String::from("hello");
    // let ref1 = &s; // ok
    // let ref2 = &s; // ok
    // println!("{}, {}", ref1, ref2);

    // this code is ok because the mutable ref is being used while
    // the immutable ones are no longer used in this scope!
    // let ref3 = &mut s;
    // println!("{}", ref3);

    // The Slice Type
    let s = String::from("Hello world!");
    let hello: &str = first_word(&s);
    println!("first_word output: {hello}");

    // string literals are slices
    let s = "Hello world!";
    let word = first_word(s);
    println!("first_word output: {hello}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

}
