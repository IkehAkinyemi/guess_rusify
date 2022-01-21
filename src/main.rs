use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guess_rusify game, guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a guess number: ");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line");

        let user_input: u32 = match user_input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            

        println!("Your input is: {}", user_input);

        match user_input.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You won!");
                break;
            },
            Ordering::Greater => println!("Way big a guess"),
            Ordering::Less => println!("That's a small guess")
        }
    }
}
