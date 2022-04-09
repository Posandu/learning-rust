struct Color(i32, i32, i32);

struct Person {
    name: String,
    age: i32,
    last_active: String,
    dead: bool,
    color: Color,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn fib(&self) -> u32 {
        self.width * self.height
    }
}

struct Shape {
    width: u32,
    height: u32,
}

impl Shape {
    fn can_handle(&self, shp: &Shape) -> bool {
        self.width > shp.width && self.height > shp.height
    }
}

struct Perimeter(u32, u32, u32, u32);

fn main() {
    let mut john = Person {
        name: String::from("John Doe"),
        age: 42,
        last_active: String::from("2018-01-01"),
        dead: true,
        color: Color(0, 40, 0),
    };

    println!(
        "{} is {} years old and was last active on {}",
        john.name, john.age, john.last_active
    );

    john.age = 43;
    john.last_active = String::from("2018-02-02");

    println!(
        "{} is {} years old and was last active on {} he is {}. His mom said {}",
        john.name,
        john.age,
        john.last_active,
        if john.dead { "dead" } else { "alive" },
        john.color.0
    );

    ////////////////////
    let jane = Person {
        name: String::from("Jane Dong"),
        dead: false,
        ..john
    };

    println!(
        "{} is {} years old and was last active on {} and is {}",
        jane.name,
        jane.age,
        jane.last_active,
        if jane.dead { "dead" } else { "alive" }
    );

    /////////////////////////////////
    let my_moms_rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "My moms rectangle is {} x {}",
        my_moms_rectangle.width, my_moms_rectangle.height
    );

    // Bad code tbh
    let my_perimeter = Perimeter(
        my_moms_rectangle.width,
        my_moms_rectangle.height,
        my_moms_rectangle.width,
        my_moms_rectangle.height,
    );

    println!(
        "My moms perimeter is {}",
        my_perimeter.0 + my_perimeter.1 + my_perimeter.2 + my_perimeter.3
    );

    println!("{:?}", my_moms_rectangle);
    dbg!(&my_moms_rectangle);

    // Methods
    let my_moms_fib = my_moms_rectangle.fib();
    println!("My moms fib is {}", my_moms_fib);
    /////////////////////////////////
    let shape = Shape {
        width: 10,
        height: 20,
    };

    let shape2 = Shape {
        width: 20,
        height: 30,
    };

    let shape3 = Shape {
        width: 3,
        height: 4,
    };

    println!("{}", shape.can_handle(&shape2));
    println!("{}", shape.can_handle(&shape3));
}
