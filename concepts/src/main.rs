use std::io;

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
}
