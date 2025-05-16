use std::io; // Obtain library for input/output

use rand::Rng; // Obtain library for random number generation

fn main() {

    // TO DO: Add menu system using match, https://www.w3schools.com/rust/rust_match.php
    
    let words = vec!["apple", "banana", "pear", "pineapple", "grape"]; // Define word array
    
    let chosen_word = obtainword(&words); // Choose random word from array

    println!("The chosen word is {}", chosen_word); // Print word to user

    let running = true; // Set running bool for game loop

    let mut guess = String::new(); // Create guess variable
    
    let mut guessed: Vec<char> = Vec::new(); // Create vector for used letters

    while running {
        displayword(chosen_word, &guessed);

        println!("Guess the word!");
        
        let mut user_input = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You said: {}", guess);
    }
}

fn obtainword<'a>(words: &'a [&str]) -> &'a str {
    // Choose random item from list
    let num = rand::thread_rng().gen_range(0..words.len());
    // Set word to item from vector
    return words[num]
}

fn displayword<'a>(word: &str, guessed: &Vec<char>) {
    let completed = true;
    for character in 0..word.len() {
        for guess in guessed {
            // if character == guess { Checks to see used letters, NEED TO FIX CHARACTER COMPARISON
                // ADD: Any used letters are displayed, if in the word
            // } else {

            // }
        }
    }
    if completed == true {
        // ADD: If all letters are displayed (check), game ends
    }
}

// Type "cargo run" in terminal to run