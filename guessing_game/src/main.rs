use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // using start..=end expression, inclusive range

    loop {
        let guess = read_guess();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn read_guess() -> u32 {
    println!("Please input your guess.");
    loop {
        let mut guess = String::new(); // mut means mutable (variables are immutable by default in rust)
        io::stdin()
            .read_line(&mut guess) // & means reference and it is immutable by default so the &mut is used to make the reference mutable
            .expect("Failed to read line");

        // move from expect to match to move from catching the error to handling it
        match guess.trim().parse() {
            Ok(num) => {
                println!("You guessed: {guess}");
                return num;
            }
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
    }
}
