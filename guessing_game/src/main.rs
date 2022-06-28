use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;


fn main() {
    println!("Guess the numebr!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the secert number is : {}", secret_number);

    loop {
        println!("Please input your guess.!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".red()),
            Ordering::Greater => println!("{}", "too big".red()),
            Ordering::Equal => {
                println!("{}", "you win!".green());
                break;
            },
        }
    }
}
