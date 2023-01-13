use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;
use std::process;

fn main() {
    _mainloop();
}

fn _mainloop() {
    let number = rand::thread_rng().gen_range(0..=100);

    loop {
        let mut guess = String::new();
        println!("Guess Number : ");
        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u8 = guess.trim().parse().expect("Type a Number");
        match guess.cmp(&number) {
            Ordering::Equal => { 
                println!("Equal, You Win");
                println!("Do you want to play again : ");
                let mut again = String::new();
                stdin().read_line(&mut again).expect("Failed");
                let again: &str = again.trim();
                if again == "yes" || again == "y" {
                    _mainloop();
                }else {
                    process::exit(1);
            }
            },
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
        }
    }
}