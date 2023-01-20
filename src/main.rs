use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;
use rand::thread_rng;
use colored::Colorize;

fn main() {
    println!("{}", "Guess Game!".truecolor(51, 0, 0));

    let mut secret_number = thread_rng().gen_range(0..=100);

    loop {
        println!("{}", "Enter a Number : ");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read Line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a Number");
                continue
            },
        };

        match guess.cmp(&mut secret_number) {
            Ordering::Greater => println!("{}", "Greater".red()),
            Ordering::Less => println!("{}", "Less".blue()),
            Ordering::Equal => {
                println!("{}", "You Win".yellow());
                break;
            }
        }
    }
}
