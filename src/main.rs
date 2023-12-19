use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess a number between 1..100");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess:");
        println!("----- press ctrl + c to quit game! -----");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("-----> your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("*** you win! ***");
                break;
            }
        }
    }

    println!("-----> secret number is: {secret_number}");
}
