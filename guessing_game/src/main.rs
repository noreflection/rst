use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("input guess:");

    loop {
        let mut guess = String::new();

        let secret_number = rand::thread_rng()
            .gen_range(1, 101);

        io::stdin().read_line(&mut guess)
            .expect("failed to read an input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("secret is: {}", secret_number);
        println!("your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("secret is less than guess!"),
            Ordering::Greater => println!("secret is greater than guess!"),
            Ordering::Equal => {
                println!("secret equals the guess!");
                break;
            }
        }
    }
}
