use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v_with_values = vec![1, 2, 3];

    println!("{:?}", v_with_values);

    v.push(5);
    v.push(6);
    v.push(7);

    let _third: &i32 = &v[2];
    match v.get(2) {
        Some(_third) => println!("The third element is {}", _third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i);
    }

    //////////
    let mut d = vec![1, 2, 3, 4, 5];
    for i in &mut d {
        *i += 50; // * is dereference operator
    }
    println!("{:?}", d);

    // Testing nested
    let nested_idk_if_this_works: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];

    println!("{:?}", nested_idk_if_this_works);

    // Strings
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    let world = String::from("world!");
    hello.push_str(&world);

    println!("{}", hello);

    let rick_roll = String::from("Never gonna give you up");
    let rick_roll_length = rick_roll.len();
    println!("{}", rick_roll_length);

    for c in rick_roll.chars() {
        let c = c.to_lowercase();

        println!("{}", c);
    }

    for c in rick_roll.bytes() {
        println!("{}", c);
    }

    println!("{}", rick_roll.contains("never"));
    println!("{}", &rick_roll[0..8]);

    // Hashmaps
    let mut scores = HashMap::new();
    // Add some values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // Show all values
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
