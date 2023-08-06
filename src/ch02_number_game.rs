use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use std::io::stdin;
use std::process::exit;

pub fn start_game() {
    println!("============Number guess game begins now !============");
    println!("Producing random number ....");
    let min = 1; let max = 100;
    let random_number = thread_rng().gen_range(min..=max);
    loop {
        println!(">input a number between {} and {} : ", min, max);
        let mut user_guess = String::new();
        stdin()
            .read_line(&mut user_guess)
            .expect("Failed reading input");

        let user_guess:i32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input cannot be parsed to integer....try again");
                continue
            }
        };
        match user_guess.cmp(&random_number){
            Ordering::Less => {println!("WRONG !! try something higher")}
            Ordering::Equal => {
                println!("============CORRECT ANSWER===========");
                exit(0)
            }
            Ordering::Greater => {println!("WRONG !! try something lower")}
        }
    }
}