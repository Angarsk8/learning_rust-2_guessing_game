use rand::Rng;
use std::cmp::Ordering;

fn random_between(from: u8, to: u8) -> u8 {
    let mut range = rand::thread_rng();
    range.gen_range(from, to)
}

fn print_guess_history(history: &mut Vec<String>, compact: bool) {
    if compact {
        println!("Guess history: {:?}", history);
        return;
    }

    println!("Here's your guess history:");
    history
        .iter()
        .enumerate()
        .for_each(|(i, guess)| println!("{}). You guessed: {}", i + 1, guess.trim()));
}

fn guess_number_cli(start_message: &str) {
    const MAX_ATTEMPTS: u8 = 20;
    const FROM: u8 = 1;
    const TO: u8 = 11;

    let mut guess = String::new();
    let mut history: Vec<String> = vec![];

    println!("{}", start_message);

    for _ in 0..MAX_ATTEMPTS {
        println!("Generating a new  random number...");
        let random_number = random_between(FROM, TO);
        println!("Input a number between {} and {}.", FROM, TO);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input.");

        history.push(format!("{}", guess));

        let parsed_guess: u8 = match guess.trim().parse() {
            Ok(number) => number,
            Err(error) => {
                println!("Incorrect input: {}", error);
                guess.clear();
                continue;
            }
        };

        match parsed_guess.cmp(&random_number) {
            Ordering::Less => println!(
                "Your guess was too low (< {}), keep trying...",
                random_number
            ),
            Ordering::Greater => println!(
                "Your guess was too high (> {}), keep trying...",
                random_number
            ),
            Ordering::Equal => {
                println!("Your nailed it!");
                print_guess_history(&mut history, false);
                return;
            }
        }

        guess.clear();
    }

    println!("Maximum number of attempts reached: {}", MAX_ATTEMPTS);
}

fn main() {
    guess_number_cli("Guess the number!");
}
