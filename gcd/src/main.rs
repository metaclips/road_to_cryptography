use std::io;

fn main() {
    let mut no1 = String::new();

    io::stdin()
        .read_line(&mut no1)
        .expect("Could not read from buffer");

    let mut no2 = String::new();
    io::stdin()
        .read_line(&mut no2)
        .expect("Could not read from buffer");

    let no2 = no2.trim();
    let no1 = no1.trim();

    let mut no1 = no1.trim().parse::<i32>().expect("invalid input");
    let mut no2 = no2.trim().parse::<i32>().expect("invalid input");

    if no1 < no2 {
        let initial_number1 = no2.clone();
        no2 = no1;
        no1 = initial_number1;
    }
    let mut answer = 0;
    euclidean_algorithm(&mut no1, &mut no2, &mut answer);
    println!("The greatest common divisor is {}", no2);
}

fn euclidean_algorithm(a: &mut i32, b: &mut i32, remainder: &mut i32) {
    *remainder = *a % *b;
    if *remainder <= 0 {
        return;
    }

    *a = *b;
    *b = *remainder;
    euclidean_algorithm(a, b, remainder)
}
