use std::io; // Obtain library for input/output

use rand::Rng; // Obtain library for random number generation

fn main() {

    let mut program_active = true;

    while program_active == true {
        println!("Main Menu\n0) Quit program\n1) Start game\n2) Change list");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let menu_choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match menu_choice {
            0 => program_active = false,
            1 => gameloop(),
            2 => println!("Coming soon!"),
            _ => println!("Unacceptable input."),
        }
    }
}

fn gameloop() {
    let words = vec!["apple", "banana", "pear", "pineapple", "grape"];
    let chosen_word = obtainword(&words); 
    let mut guessed: Vec<char> = Vec::new();

    loop {
        // Display word & already guessed letters
        displayword(chosen_word, &guessed);
        if !guessed.is_empty() {
            print!("Guessed letters: ");
            displayguessed(&guessed);
        }
        
        let mut guess = String::new();

        println!("Guess a letter: ");

        // Obtain user guess
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let trimmed = guess.trim();

        if trimmed.len() != 1 {
            println!("Only type one letter.");
        }

        let letter = trimmed.chars().next().unwrap();

        if !letter.is_alphabetic() {
            println!("Please enter a valid letter (a-z).");
            continue;
        }

        if guessed.contains(&letter) {
            println!("You've already guessed '{}'. Try a new letter.", letter);
            continue;
        }

        guessed.push(letter);

        // TO DO: Check if word is fully guessed, and end game if so
    }
}

fn obtainword<'a>(words: &'a [&str]) -> &'a str {
    // Choose random item from list
    let num = rand::thread_rng().gen_range(0..words.len());
    // Set word to item from vector
    return words[num]
}

fn displayword(word: &str, guessed: &Vec<char>) {
    for character in word.chars() {
        if guessed.contains(&character) {
            print!("{} ", character)
        } else {
            print!("_ ");
        }
    }
    println!();
}

fn displayguessed(guessed: &Vec<char>) {
    for (i, element) in guessed.iter().enumerate() {
        if i == guessed.len() - 1{
            println!("{}", element);
        } else {
            print!("{}, ", element);
        }
    }
}
// Type "cargo run" in terminal to run