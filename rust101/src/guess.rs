use rand::prelude::*;
use std::{cmp::Ordering, io};

pub fn guess_game() {
    println!("guess the number!");

    // answer in here...
    let mut rng = rand::rng();
    let nums: Vec<u32> = (1..=100).collect();
    let answer = nums.choose(&mut rng).unwrap();

    loop {
        println!("input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line...");

        println!("guess number: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(answer) {
            Ordering::Less => println!("Too small!!: {}", answer - guess),
            Ordering::Greater => println!("Too big!!: {}", guess - answer),
            Ordering::Equal => {
                println!("WIN");
                break;
            }
        }
    }
}
