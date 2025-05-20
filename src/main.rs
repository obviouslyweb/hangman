use std::io; // Obtain library for input/output

use rand::Rng; // Obtain library for random number generation

fn main() {

    clearscreen::clear().expect("failed to clear screen");

    let mut program_active = true;
    let mut lives = 6;

    while program_active == true {
        println!("o<-< RUSTMAN Main Menu >->o\n0) Quit program\n1) Start game\n2) Change list\n3) Change allowed missed guesses");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let menu_choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                clearscreen::clear().expect("failed to clear screen");
                println!("Invalid input. Please enter a number.\n");
                continue;
            }
        };

        match menu_choice {
            0 => program_active = false,
            1 => gameloop(lives),
            2 => {
                clearscreen::clear().expect("failed to clear screen");
                println!("Functionality coming soon!\n");
            }
            3 => lives = changelives(lives),
            _ => {
                clearscreen::clear().expect("failed to clear screen");
                println!("Unacceptable input; please choose a menu option.\n");
            }
        }
    }
}

fn gameloop(lives: i32) {

    clearscreen::clear().expect("failed to clear screen");
    
    let words = vec!["apple", "banana", "pear", "pineapple", "grape"];
    let chosen_word = obtainword(&words); 
    let mut activelives = lives;
    let mut guessed: Vec<char> = Vec::new();
    let gameendcondition: i32; // 0 = error, 1 = win, 2 = lose

    loop {
        // Display word
        displayword(chosen_word, &guessed);
        
        // Display already guessed letters & lives
        if !guessed.is_empty() {
            print!("Lives: {}, Guessed letters: ", activelives);
            displayguessed(&guessed);
        } else {
            print!("Lives: {}\n", activelives);
        }

        println!("");
        let mut guess = String::new();
        println!("Guess a letter: ");

        // Obtain user guess
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let trimmed = guess.trim();

        if trimmed.len() != 1 {
            clearscreen::clear().expect("failed to clear screen");
            println!("Only type one letter.");
            continue;
        }

        let letter = trimmed.chars().next().unwrap();

        if !letter.is_alphabetic() {
            clearscreen::clear().expect("failed to clear screen");
            println!("Please enter a valid letter (a-z).");
            continue;
        }

        if guessed.contains(&letter) {
            clearscreen::clear().expect("failed to clear screen");
            println!("You've already guessed '{}'. Try a new letter.", letter);
            continue;
        }
        
        // Add letter to guessed letters
        guessed.push(letter);

        // Increment lives if not in word
        if !chosen_word.contains(letter) {
            activelives -= 1;
        }

        // Check if game needs to end
        let completed = checkword(chosen_word, &guessed);
        if completed || activelives == 0 {
            if completed {
                gameendcondition = 1;
                break;
            } else {
                gameendcondition = 2;
                break;
            }
        }

        clearscreen::clear().expect("failed to clear screen");
    }

    clearscreen::clear().expect("failed to clear screen");

    if gameendcondition == 1 {
        println!("[ YOU WON! ]\nYou guessed the word '{}' in {} guesses.\nPress ENTER to continue.", chosen_word, guessed.len());
    } else if gameendcondition == 2 {
        println!("[ You lost... ]\nThe word was '{}', and you made {} guesses.\nPress ENTER to continue.", chosen_word, guessed.len());
    } else {
        println!("An error occured and the game has ended.\nIf you see this, please let me know!\nPress ENTER to continue.");
    }

    let mut empty = String::new();
    io::stdin().read_line(&mut empty).expect("Failed to read line");

    clearscreen::clear().expect("failed to clear screen");

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

fn checkword(word: &str, guessed: &Vec<char>) -> bool {
    let mut completed = true;
    for character in word.chars() {
        if !guessed.contains(&character) {
            completed = false;
        }
    }
    if completed {
        return true;
    } else {
        return false;
    }
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

fn changelives(mut lives: i32) -> i32 {

    clearscreen::clear().expect("failed to clear screen");

    println!("The current number of missed guesses before you lose the game is currently {}.\n", lives);
    println!("How many missed guesses do you want to allow? (integer)");
    let mut proposed_lives = String::new();
    io::stdin().read_line(&mut proposed_lives).expect("Failed to read line");
    let mut lives_changed = false;
    match proposed_lives.trim().parse::<i32>() {
        Ok(_num) => lives_changed = true,
        Err(_) => println!("That is not an acceptable number. Please try again with an integer."),
    }

    clearscreen::clear().expect("failed to clear screen");

    if lives_changed {
        if proposed_lives.trim().parse::<i32>().unwrap_or(lives) < 25 {
            lives = proposed_lives.trim().parse::<i32>().unwrap_or(lives);
            println!("Allowed missed guesses has been changed to {} for future games.\nPress ENTER to continue.", proposed_lives.trim());
        } else {
            println!("Too big! Allowed missed guesses must not match or exceed the total length of the alphabet (26).\nPress ENTER to continue.")
        }
    } else {
        println!("That is not an acceptable number. Please try again with an integer.\nAllowed missed guesses remains at {}.\nPress ENTER to continue.", lives);
    }

    let mut empty = String::new();
    io::stdin().read_line(&mut empty).expect("Failed to read line");

    clearscreen::clear().expect("failed to clear scrWeen");

    return lives;
}
// Type "cargo run" in terminal to run