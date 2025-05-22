//   ____   _   _  ____  _____  __  __     _     _   _ 
//  |  _ \ | | | |/ ___||_   _||  \/  |   / \   | \ | |
//  | |_) || | | |\___ \  | |  | |\/| |  / _ \  |  \| |
//  |  _ < | |_| | ___) | | |  | |  | | / ___ \ | |\  |
//  |_| \_\ \___/ |____/  |_|  |_|  |_|/_/   \_\|_| \_|

// Hangman Game created in Rust
// Created by Connor Butterfield (TWC)
// Part of a two-week sprint exploration project to learn Rust

// Define necessary libraries
use std::{io};
use rand::Rng;
use clearscreen;

// Define WordList structure
struct WordList {
    name: String,
    words: Vec<String>
}

// Main function
fn main() {

    // Clear screen
    clearscreen::clear().expect("failed to clear screen");

    // Create necessary variables (word list, chosen word list, active program, lives, etc.)
    let mut word_lists = createwordlist();
    let mut chosen_word_list = 0;
    let mut program_active = true;
    let mut lives = 6;

    // Main menu loop
    while program_active == true {
        println!("o<-< RUSTMAN Main Menu >->o\n0) Quit program\n1) Start game ('{}', {} missed allowed)\n2) Word list settings\n3) Change allowed missed guesses", &word_lists[chosen_word_list].name, lives);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Ensure input can be parsed
        let menu_choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                clearscreen::clear().expect("failed to clear screen");
                println!("Invalid input. Please enter a number.\n");
                continue;
            }
        };

        // Menu choice decider
        match menu_choice {
            0 => program_active = false, // Close program
            1 => {
                // Start main game using provided settings
                clearscreen::clear().expect("failed to clear screen");
                gameloop(lives, chosen_word_list, &word_lists)
            }
            2 => {
                // Word list options menu loop
                loop {

                    // Display currently selected word list contents
                    clearscreen::clear().expect("failed to clear screen");
                    println!("The current selected word list is '{}'.", word_lists[chosen_word_list].name);
                    print!("It includes the following words: ");
                    for (i, word) in word_lists[chosen_word_list].words.iter().enumerate() {
                        if i == word_lists[chosen_word_list].words.len() - 1 {
                            println!("{}", word);
                        } else {
                            print!("{}, ", word);
                        }
                    }
                    
                    println!("\n0) Return to main menu\n1) Change word list\n2) Create new word list");

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
                        0 => break,
                        1 => {
                            // Change word list
                            chosen_word_list = changewords(chosen_word_list, &word_lists);
                        }
                        2 => {
                            // Create new list and add to lists structure
                            addtowordlist(&mut word_lists);
                        }
                        _ => {
                            clearscreen::clear().expect("failed to clear screen");
                            println!("Unacceptable input; please choose a menu option.\n");
                        }
                    }
                }

                clearscreen::clear().expect("failed to clear screen");
            }
            3 => lives = changelives(lives),
            _ => {
                clearscreen::clear().expect("failed to clear screen");
                println!("Unacceptable input; please choose a menu option.\n");
            }
        }
    }
}

fn gameloop(lives: i32, chosen_word_list: usize, word_list: &Vec<WordList>) {

    // Declare game variables
    let chosen_word = obtainword(&word_list[chosen_word_list].words);
    let mut activelives = lives;
    let mut guessed: Vec<char> = Vec::new();
    let gameendcondition: i32; // 0 = error, 1 = win, 2 = lose

    // Main game loop
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

        // Verify input is acceptable
        let letter = trimmed.chars().next().unwrap();
        if !letter.is_alphabetic() {
            clearscreen::clear().expect("failed to clear screen");
            println!("Please enter a valid letter (a-z).");
            continue;
        }

        // Verify input wasn't already guessed
        if guessed.contains(&letter) {
            clearscreen::clear().expect("failed to clear screen");
            println!("You've already guessed '{}'. Try a new letter.", letter);
            continue;
        }
        
        // If previous checks pass (don't trigger), add letter to guessed letters
        guessed.push(letter);

        // Increment lives if not in word
        if !chosen_word.contains(letter) {
            activelives -= 1;
        }

        // Check if game needs to end (word was guessed or lives are gone)
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

    // Display game end screen depending on end condition
    if gameendcondition == 1 {
        println!("[ YOU WON! ]\nYou guessed the word '{}' in {} guesses.\nPress ENTER to continue.", chosen_word, guessed.len());
    } else if gameendcondition == 2 {
        println!("[ You lost... ]\nThe word was '{}', and you made {} guesses.\nPress ENTER to continue.", chosen_word, guessed.len());
    } else {
        println!("An error occured and the game has ended.\nIf you see this, please let me know!\nPress ENTER to continue.");
    }

    // Press ENTER handler
    let mut empty = String::new();
    io::stdin().read_line(&mut empty).expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

}

fn obtainword<'a>(words: &'a [String]) -> &'a str {
    // Choose random word/phrase from given word list
    let num = rand::thread_rng().gen_range(0..words.len());
    // Return chosen word
    return &words[num]
}

fn displayword(word: &str, guessed: &Vec<char>) {
    for character in word.chars() { // For each character in the given word...
        if guessed.contains(&character) { // If it's already been guessed...
            print!("{} ", character) // Display it normally
        } else if character == ' ' { // If it is a space...
            print!("  "); // Display empty
        } else { // If it hasn't already been guessed...
            print!("_ "); // Display as an underscore
        }
    }
    println!();
}

fn checkword(word: &str, guessed: &Vec<char>) -> bool {
    let mut completed = true;
    for character in word.chars() { // For each character in the word...
        if !guessed.contains(&character) { // If it HASN'T been guessed...
            if character != ' ' { // ...and it's NOT a space...
                completed = false; // ...set completed to FALSE
            }
        }
    } 
    return completed; // Return the value of completed
}

fn displayguessed(guessed: &Vec<char>) {
    // For displaying the characters ALREADY guessed, regardless of accuracy
    for (i, element) in guessed.iter().enumerate() { // For each element in guessed...
        if i == guessed.len() - 1{ // If it's the last one in the array...
            println!("{}", element); // Display by itself
        } else { // If it's not the last one in the array...
            print!("{}, ", element); // Display with a comma for nice formatting :)
        }
    }
}

fn changelives(mut lives: i32) -> i32 {
    clearscreen::clear().expect("failed to clear screen");

    // Display current lives and explain usage
    println!("The current number of missed guesses before you lose the game is currently {}.\n", lives);
    println!("How many missed guesses do you want to allow? (positive integer below 26)");

    // Obtain user input
    let mut proposed_lives = String::new();
    io::stdin().read_line(&mut proposed_lives).expect("Failed to read line");
    let mut lives_changed = false;

    // Verify that provided number can be parsed as an integer
    match proposed_lives.trim().parse::<i32>() {
        Ok(_num) => lives_changed = true,
        Err(_) => println!("That is not an acceptable number. Please try again with an integer."),
    }

    clearscreen::clear().expect("failed to clear screen");

    // If the number can be parsed...
    if lives_changed {
        // The next line looks complicated, but it's really not. Essentially, if the number provided is less than 26 and positive, then it will pass.
        if (proposed_lives.trim().parse::<i32>().unwrap_or(lives) < 26) && (proposed_lives.trim().parse::<i32>().unwrap_or(lives) > 0)  {
            // Set the current lives to the user's input
            lives = proposed_lives.trim().parse::<i32>().unwrap_or(lives);
            println!("Allowed missed guesses has been changed to {} for future games.\nPress ENTER to continue.", proposed_lives.trim());
        } else {
            // If conditions didn't pass, then the number wasn't allowed and nothing is changed
            println!("Prohibited number; allowed missed guesses must be more than 0 and not match or exceed the total length of the alphabet (26).\nPress ENTER to continue.")
        }
    } else {
        println!("That is not an acceptable number. Please try again with an integer.\nAllowed missed guesses remains at {}.\nPress ENTER to continue.", lives);
    }

    // Press ENTER handler
    let mut empty = String::new();
    io::stdin().read_line(&mut empty).expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    // Return value of lives (whether updated or not)
    return lives;
}

fn changewords(mut current_word_list: usize, word_lists: &Vec<WordList>) -> usize {
    clearscreen::clear().expect("failed to clear screen");

    // Display the current lists the program has
    println!("The following are all the lists you can use:\n");
    for (index, list) in word_lists.iter().enumerate() {
        // Print list index and name...
        print!("{}: {} | ", index + 1, list.name);
        // ...and then the words in that list
        for (i, word) in word_lists[index].words.iter().enumerate() {
            // Uses similar conditional formatting for nice looking formatting
            if i == word_lists[index].words.len() - 1 {
                print!("{}", word);
            } else {
                print!("{}, ", word);
            }
        }
        println!();
    }

    // Obtain user input for new list selection
    println!("\nPlease input the number of the list you'd like to use.");
    let mut new_list_input = String::new();
    io::stdin().read_line(&mut new_list_input).expect("Failed to read line");

    // Determine if user input can be parsed; if not, quit early and change nothing
    let list_choice: i32 = match new_list_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            clearscreen::clear().expect("failed to clear screen");
            println!("Invalid input. Please enter a number.\n");
            return current_word_list;
        }
    };

    // If the input can be parsed and it cooresponds to a list item...
    if list_choice > 0 && (list_choice as usize) <= word_lists.len() {
        // Set the current selected list as the user selected one
        let chosen_index = (list_choice - 1) as usize;
        clearscreen::clear().expect("failed to clear screen");
        println!("Chosen word list has been changed to '{}'.\nPress ENTER to continue.", word_lists[chosen_index].name);
        current_word_list = chosen_index;
    } else {
        // Otherwise, disregard and quit
        clearscreen::clear().expect("failed to clear screen");
        println!("Invalid selection; that index did not match any provided options.\nWord list remains as '{}'.\nPress ENTER to continue.", word_lists[current_word_list].name);
    }

    // Press ENTER handler
    let mut empty = String::new();
    io::stdin().read_line(&mut empty).expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    return current_word_list;
}

fn addtowordlist(all_lists: &mut Vec<WordList>) {
    clearscreen::clear().expect("failed to clear screen");

    // Get new list name
    println!("Please enter a name for your new word list:");
    let mut list_name = String::new();
    io::stdin().read_line(&mut list_name).expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    // Create string vector for holding added words
    let mut words_to_add: Vec<String> = Vec::new();

    // Get new list items
    clearscreen::clear().expect("failed to clear screen");
    loop { // This code loops until the user says they're done inserting new words/phrases
        println!("Please enter words for the new word list. Each entry will count as a new word.\nTo finish adding words, type '/'.\nCurrently added words: {:?}", words_to_add);
        
        // Obtain word to add
        let mut new_list_item = String::new();
        io::stdin().read_line(&mut new_list_item).expect("Failed to read line");

        // Trim input to remove new line character
        let trimmed = new_list_item.trim();

        // If the user didn't want to quit (!= "/"), proceed 
        if trimmed != "/" {
            // If the user input is empty or is completely made up of spaces...
            if trimmed.is_empty() || !trimmed.chars().any(|c| c.is_alphabetic()) {
                // Prevent entry and restart loop
                clearscreen::clear().expect("failed to clear screen");
                println!("Input cannot be empty or only spaces. Please enter a valid word/phrase.");
                continue;
            }

            // Otherwise, continue checks
            let mut allowed_phrase = true;

            // For each character in the user provided phrase...
            for c in trimmed.chars() {
                // ...if any AREN'T alphabetic or a space, disallow it
                if !(char::is_alphabetic(c) || c == ' ') {
                    allowed_phrase = false;
                }
            }

            // Checks to ensure that the last check passed
            if allowed_phrase {
                // Add word/phrase to the word list
                words_to_add.push(trimmed.to_string());
                clearscreen::clear().expect("failed to clear screen");
            } else {
                // Notify user that content wasn't allowed
                clearscreen::clear().expect("failed to clear screen");
                println!("'{}' is not an allowed word and was not added. Please only use alphabetical characters and spaces.", new_list_item.trim());
            }

            // The two above checks could probably be combined in retrospect; might do that eventually

        } else {
            // If the user wanted to quit by inputting "/", break the loop to move on
            break;
        }
    }

    // OUTSIDE OF LOOP: The user is done inputting words into the list, so now we test checks for creation
    
    // If the word list isn't empty...
    if words_to_add.len() != 0 {
        // ...then officially push the new word list to the full vector
        all_lists.push(WordList {
                name: list_name.trim().to_string(),
                words: words_to_add.iter().map(|s| s.to_string()).collect(),
        });

        // Notify user of completion
        clearscreen::clear().expect("failed to clear screen");
        println!("Added list '{}' to word list options. You can play with this list by selecting it in the word list options menu.\n\n{} | {:?}\n\nPress ENTER to continue.", list_name.trim(), list_name.trim(), words_to_add);

        // Press ENTER handler
        let mut empty = String::new();
        io::stdin().read_line(&mut empty).expect("Failed to read line");
        clearscreen::clear().expect("failed to clear screen");

    } else {
        // If the list IS empty, notify user of cancelled creation
        clearscreen::clear().expect("failed to clear screen");
        println!("Provided list had no entries! No list was created.\nPress ENTER to continue.");

        // Press ENTER handler
        let mut empty = String::new();
        io::stdin().read_line(&mut empty).expect("Failed to read line");
        clearscreen::clear().expect("failed to clear screen");
    }
}

fn createwordlist() -> Vec<WordList> {
    // Default starting word lists, created at runtime
    return vec![
        WordList {
            name: "Fruits".to_string(),
            words: vec!["apple", "banana", "pear", "pineapple", "grape", "blackberry", "guava", "peach", "orange", "dragonfruit", "tomato"]
            .iter().map(|s| s.to_string()).collect(),
        },
        WordList {
            name: "Computers".to_string(),
            words: vec!["macbook", "windows", "keyboard", "monitor", "speaker", "phone", "zephyrus", "microsoft", "nvidia", "button", "stack overflow"]
            .iter().map(|s| s.to_string()).collect(),
        },
        WordList {
            name: "Games".to_string(),
            words: vec!["cult of the lamb", "another crabs treasure", "minecraft", "splatoon", "a hat in time", "the legend of zelda", "super mario odyssey", "risk of rain returns", "helldivers two", "mariokart", "sonic adventure"]
            .iter().map(|s| s.to_string()).collect(),
        }
    ];
}

// End of program