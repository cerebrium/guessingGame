use std::io;

fn main() {
    println!("Hello, welcome to guessing game!");
    let number: u32 = 6;

    let mut guess_response = true;

    loop {
        if guess_response != true{
            break;
        }
        guess_response = generate_guesses(number);
    }

}

fn generate_guesses(number: u32) -> bool{
    println!("inside the generate guesses function");
    let mut guess = String::new();
    println!("guess the number (0 - 10): ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        
    return handle_guess_response(guess, number);
}
    
fn handle_guess_response(guess: String, number: u32) -> bool{
    let guess = guess.trim().parse::<u32>();

    let guess = match guess {
        Ok(num) => num,
        Err(e) => {
            println!("Please enter a number: {}", e);
            return true;
        }
    };

    if guess == number {
        println!("You guessed the number!");
        return false;
    } else if guess > number {
        println!("Your guess is too high!");
        return true;
    } else {
        println!("Your guess is too low!");
        return true;
    }
}
