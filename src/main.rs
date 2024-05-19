use std::{cmp::Ordering, io};

use rand::Rng;

use colored::*;

fn main() {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let random = rand::thread_rng().gen_range(0..10);
        match guess.cmp(&random) {
            Ordering::Less => println!("{}", "too Small".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You Won!".green());
                break;
            }
        }
    }
}
