use std::io;

fn main() {
    let mut no1 = String::new();
    let mut no2 = String::new();

    println!("Please input first number:");
    io::stdin()
        .read_line(&mut no1)
        .expect("Could not first variable read from IO");

    println!("Please input second number:");
    io::stdin()
        .read_line(&mut no2)
        .expect("Could not second variable read from IO");

    let no1 = no1.trim().parse::<i64>().expect("This is not an integer");
    let no2 = no2.trim().parse::<i64>().expect("This is not an integer");

    println!(
        "The greatest common divisor of {} and {} is {}",
        no1,
        no2,
        euclid(no1, no2)
    );
}

fn euclid(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    return euclid(b, modulus(a, b));
}

fn modulus(a: i64, b: i64) -> i64 {
    a % b
}
