use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let min_num = 1;
    let max_num = 100;
    let num_tries = 5;
    let upper_range = min_num + max_num;
    let mut guess_count = 0;

    println!("Guess the number between {} and {} in {} tries!", min_num, max_num, num_tries);

    let secret_number = rand::thread_rng()
        .gen_range(min_num, upper_range);

    loop {
        guess_count = guess_count + 1;

        
        if guess_count > num_tries {
            println!("Out of guesses!");
            break;
        }

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

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
