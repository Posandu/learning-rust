fn main() {
    let mut str = String::from("Hello");
    println!("{}", str);
    // Push a string to the end of the string
    str.push_str(" World");
    println!("{}", str); // Prints "Hello World"

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // s2 is a copy of s1 we need to use this instead of s2 = s1;
    println!("s1 = {}, s2 = {}", s1, s2);

    let str = give_ownership();
    println!("{}", str);
    take_ownership(str);
    // println!("{}", str); (The take_ownership function takes ownership of the string and therefore we can't access it anymore) RIP

    let another_str = String::from("Hello");
    let a_new = transfer_ownership(another_str);
    println!("{}", a_new);

    let s = String::from("hello");
    let (s, len) = calculate_len(s);
    println!("{} {}", s, len);

    /////////////////////////
    println!("References");

    references();
}

fn give_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn transfer_ownership(str: String) -> String {
    str
}

fn calculate_len(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}

///////////////////////////////////////

fn references() {
    let string = String::from("TEST");
    let length = calculate_length(&string);
    println!("The length of '{}' is {}.", string, length); // yeye

    ////////////////////
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    println!("{}", first_word(&s));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(str: &mut String) {
    str.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // if the item is a space
            return &s[0..i];
        }
    }

    &s[..]
}
