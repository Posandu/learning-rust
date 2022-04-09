use rand::Rng;
use std::io;

fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn rickroll() {
    let lyrics = [
        "Never gonna give you up",
        "Never gonna let you down",
        "Never gonna run around and desert you",
        "Never gonna make you cry",
        "Never gonna say goodbye",
        "Never gonna tell a lie and hurt you",
    ];

    for line in lyrics.iter() {
        println!("{}", line);
    }
}

fn main() {
    // Mutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The max points is: {}", MAX_POINTS);

    // Scopes
    let food = "Vegetables";
    println!("I'm eating {}", food);

    {
        let food = "Fruit";
        println!("I'm eating {}", food);
    }

    // Data types
    let thirty_two: u32 = 32;
    let two_point_five: f32 = 2.5;
    let is_active: bool = true;

    println!("Thirty two: {}", thirty_two);
    println!("Two point five: {}", two_point_five);
    println!("Is active: {}", is_active);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_without_type = (500, 6.4, 1);
    let (x, y, z) = tup_without_type;

    println!("Tuple: {} {} {}", tup.0, tup.1, tup.2);
    println!("Tuple without type: {} {} {}", x, y, z);

    // Arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_without_type = [1, 2, 3, 4, 5];
    let item = arr[0];

    println!(
        "Array: {} {} {} {} {}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );
    println!(
        "Array without type: {} {} {} {} {}",
        arr_without_type[0],
        arr_without_type[1],
        arr_without_type[2],
        arr_without_type[3],
        arr_without_type[4]
    );
    println!("Item: {}", item);

    let mut arr_index = String::new();

    println!("Enter an index: ");

    io::stdin()
        .read_line(&mut arr_index)
        .expect("Failed to read line");

    let arr_index: usize = arr_index.trim().parse().expect("Please type a number");
    println!("The item at index {} is: {}", arr_index, arr[arr_index]);

    // Functions
    let rand_n = rand::thread_rng().gen_range(1..100);

    println!("Fibonacci of {}: {}", rand_n, fib(5));
    rickroll();

    // Comments
    // Like this
    // Ok time over

    // Loops
    let mut iterate = 0;

    loop {
        iterate += 1;

        if iterate == 10 {
            break;
        }

        if iterate % 2 == 0 {
            continue;
        }

        println!("Iteration: {}", iterate);
    }

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value is: {}", element);
    }

    // Umm python style
    for i in 0..5 {
        println!("The value is: {}", i);
    }

    for i in (0..5).rev() {
        println!("The value is: {}", i);
    }
}
