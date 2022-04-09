enum IP {
    V6(String),
    V4(String),
}

#[derive(Debug)]
enum States {
    Orio,
}

enum Coin {
    Dollar,
    Euro,
    Bicoin,
    Ethereum,
    State(States),
}

fn value_in_rupees(coin: Coin) -> f64 {
    match coin {
        Coin::Dollar => 300.36,
        Coin::Euro => 44.23,
        Coin::Bicoin => 300000000000.23,
        Coin::Ethereum => {
            println!("Pay Gas to get result");
            0.0
        }
        Coin::State(state) => {
            println!("State quarter from {:?}", state);
            0.0
        }
    }
}

fn main() {
    let my_ip = IP::V4(String::from("1.123.2.3330")); // Not an IP but who cares?
    let google_ip = IP::V6(String::from("adasdas:asdasdas:ASdasdasd:asdas")); // idk

    // Let's see what it does 🤔
    match my_ip {
        IP::V4(ip) => println!("IPv4: {}", ip),
        IP::V6(ip) => println!("IPv6: {}", ip),
    }

    match google_ip {
        IP::V4(ip) => println!("IPv4: {}", ip),
        IP::V6(ip) => println!("IPv6: {}", ip),
    }

    /////////////////////
    // This is the most confusing part of the code
    println!("Dollar value in rupees: {}", value_in_rupees(Coin::Dollar));
    println!("Euro value in rupees: {}", value_in_rupees(Coin::Euro));
    println!("Bicoin value in rupees: {}", value_in_rupees(Coin::Bicoin));
    println!(
        "Ethereum value in rupees: {}",
        value_in_rupees(Coin::Ethereum)
    );
    println!(
        "State quarter value in rupees: {}",
        value_in_rupees(Coin::State(States::Orio))
    );

    let role = 42;
    match role {
        1 => println!("You are a mom"),
        2 => println!("You are a admin"),
        3 => println!("You are a super admin"),
        _ => println!("You are a user"), // This is the default
    }
}