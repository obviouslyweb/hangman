use std::{io}; // Obtain library for input/output
use rand::Rng; // Obtain library for random number generation
use clearscreen;

// Define WordList structure
struct WordList {
    name: String,
    words: Vec<String>
}

fn main() {

    // Clear screen
    clearscreen::clear().expect("failed to clear screen");

    // Create necessary variables (word list, chosen word list, active program, lives, etc.)
    let mut word_lists = createwordlist();
    let mut chosen_word_list = 0;
    let mut program_active = true;
    let mut lives = 6;

    while program_active == true {
        println!("o<-< RUSTMAN Main Menu >->o\n0) Quit program\n1) Start game ('{}', {} missed allowed)\n2) Word list settings\n3) Change allowed missed guesses", &word_lists[chosen_word_list].name, lives);

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
            1 => {
                clearscreen::clear().expect("failed to clear screen");
                gameloop(lives, chosen_word_list, &word_lists)
            }
            2 => {
                loop {
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
                            chosen_word_list = changewords(chosen_word_list, &word_lists);
                        }
                        2 => {
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

    let chosen_word = obtainword(&word_list[chosen_word_list].words);
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

fn obtainword<'a>(words: &'a [String]) -> &'a str {
    // Choose random item from list
    let num = rand::thread_rng().gen_range(0..words.len());
    // Set word to item from vector
    return &words[num]
}

fn displayword(word: &str, guessed: &Vec<char>) {
    for character in word.chars() {
        if guessed.contains(&character) {
            print!("{} ", character)
        } else if character == ' ' {
            print!("  ");
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
            if character != ' ' {
                completed = false;
            }
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
    println!("How many missed guesses do you want to allow? (positive integer below 26)");
    let mut proposed_lives = String::new();
    io::stdin().read_line(&mut proposed_lives).expect("Failed to read line");
    let mut lives_changed = false;
    match proposed_lives.trim().parse::<i32>() {
        Ok(_num) => lives_changed = true,
        Err(_) => println!("That is not an acceptable number. Please try again with an integer."),
    }

    clearscreen::clear().expect("failed to clear screen");

    if lives_changed {
        if (proposed_lives.trim().parse::<i32>().unwrap_or(lives) < 26) && (proposed_lives.trim().parse::<i32>().unwrap_or(lives) > 0)  {
            lives = proposed_lives.trim().parse::<i32>().unwrap_or(lives);
            println!("Allowed missed guesses has been changed to {} for future games.\nPress ENTER to continue.", proposed_lives.trim());
        } else {
            println!("Prohibited number; allowed missed guesses must be more than 0 and not match or exceed the total length of the alphabet (26).\nPress ENTER to continue.")
        }
    } else {
        println!("That is not an acceptable number. Please try again with an integer.\nAllowed missed guesses remains at {}.\nPress ENTER to continue.", lives);
    }

    // Press ENTER handler
    let mut empty = String::new();
    io::stdin().read_line(&mut empty).expect("Failed to read line");
    clearscreen::clear().expect("failed to clear screen");

    return lives;
}

fn changewords(mut current_word_list: usize, word_lists: &Vec<WordList>) -> usize {
    clearscreen::clear().expect("failed to clear screen");

    println!("The following are all the lists you can use:\n");

    for (index, list) in word_lists.iter().enumerate() {
        print!("{}: {} | ", index + 1, list.name);
        for (i, word) in word_lists[index].words.iter().enumerate() {
        if i == word_lists[index].words.len() - 1 {
            print!("{}", word);
        } else {
            print!("{}, ", word);
        }
    }
        println!();
    }

    println!("\nPlease input the number of the list you'd like to use.");

    let mut new_list_input = String::new();

    io::stdin().read_line(&mut new_list_input).expect("Failed to read line");

    let list_choice: i32 = match new_list_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            clearscreen::clear().expect("failed to clear screen");
            println!("Invalid input. Please enter a number.\n");
            return current_word_list;
        }
    };

    if list_choice > 0 && (list_choice as usize) <= word_lists.len() {
        let chosen_index = (list_choice - 1) as usize;
        clearscreen::clear().expect("failed to clear screen");
        println!("Chosen word list has been changed to '{}'.\nPress ENTER to continue.", word_lists[chosen_index].name);
        current_word_list = chosen_index;
    } else {
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
    loop {
        println!("Please enter words for the new word list. Each entry will count as a new word.\nTo finish adding words, type '/'.\nCurrently added words: {:?}", words_to_add);
        let mut new_list_item = String::new();
        io::stdin().read_line(&mut new_list_item).expect("Failed to read line");
        let trimmed = new_list_item.trim();
        if trimmed != "/" {
            if trimmed.is_empty() || !trimmed.chars().any(|c| c.is_alphabetic()) {
                clearscreen::clear().expect("failed to clear screen");
                println!("Input cannot be empty or only spaces. Please enter a valid word/phrase.");
                continue;
            }

            let mut allowed_phrase = true;
            for c in trimmed.chars() {
                if !(char::is_alphabetic(c) || c == ' ') {
                    allowed_phrase = false;
                }
            }
            if allowed_phrase {
                words_to_add.push(trimmed.to_string());
                clearscreen::clear().expect("failed to clear screen");
            } else {
                clearscreen::clear().expect("failed to clear screen");
                println!("'{}' is not an allowed word and was not added. Please only use alphabetical characters and spaces.", new_list_item.trim());
            }
        } else {
            break;
        }
    }

    if words_to_add.len() != 0 {

        all_lists.push(WordList {
                name: list_name.trim().to_string(),
                words: words_to_add.iter().map(|s| s.to_string()).collect(),
        });

        clearscreen::clear().expect("failed to clear screen");
        println!("Added list '{}' to word list options. You can play with this list by selecting it in the word list options menu.\n\n{} | {:?}\n\nPress ENTER to continue.", list_name.trim(), list_name.trim(), words_to_add);

        // Press ENTER handler
        let mut empty = String::new();
        io::stdin().read_line(&mut empty).expect("Failed to read line");
        clearscreen::clear().expect("failed to clear screen");

    } else {
        clearscreen::clear().expect("failed to clear screen");
        println!("Provided list had no entries! No list was created.\nPress ENTER to continue.");

        // Press ENTER handler
        let mut empty = String::new();
        io::stdin().read_line(&mut empty).expect("Failed to read line");
        clearscreen::clear().expect("failed to clear screen");
    }
}

fn createwordlist() -> Vec<WordList> {
    return vec![
        WordList {
            name: "Fruits".to_string(),
            words: vec!["apple", "banana", "pear", "pineapple", "grape", "blackberry", "guava", "peach", "orange"]
            .iter().map(|s| s.to_string()).collect(),
        },
        WordList {
            name: "Computers".to_string(),
            words: vec!["macbook", "windows", "keyboard", "monitor", "speaker"]
            .iter().map(|s| s.to_string()).collect(),
        },
        WordList {
            name: "Games".to_string(),
            words: vec!["cult of the lamb", "another crabs treasure", "minecraft", "splatoon", "a hat in time"]
            .iter().map(|s| s.to_string()).collect(),
        }
    ];
}

// Type "cargo run" in terminal to run, "cargo build" to test compile