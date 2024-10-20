use std::{
    cmp::Ordering,
    io::{self, Write},
};

use rand::Rng;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess: ");

        let mut guess = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(u32) => u32,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("U got me.");
                break;
            }
            Ordering::Less => println!("TOO SMALL!"),
        }
    }
}
