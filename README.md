## Overview

**Project Title**: Rustman

**Project Description**: The iconic game of Hangman, replicated in Rust. Guess words chosen from a random list one letter at a time; however, any incorrect guesses will hurt your lives. Run out of lives, and you're done for! Also includes the ability to change the number of allowed missed guesses (lives) for each game, and the ability to choose from different themes for the word to guess (as well as create word lists yourself to play).

**Project Goals**: Further my understanding of the Rust programming language, syntax, functionality, and limitations by creating a game entirely using Rust and libraries created for it within two weeks.

## Instructions for Build and Use

Steps to build and/or run the software:

### Using Executable (without Rust installed)
1. [Download the latest build published on GitHub here.](https://github.com/obviouslyweb/rustman/releases)
2. Run `rustman.exe`. Windows may give you a warning as the program isn't signed; if so, press 'More info' and 'Run anyway' to proceed.

### Using Command Prompt / Terminal
1. If not already installed, [download and install the most recent version of Rust](https://www.rust-lang.org/tools/install). This should install all the necessary components (including Cargo).
2. Download the repository contents to your computer. You can either download the repository contents as a .ZIP file, or use git / GitHub Desktop to copy the repository to your device.
3. Open a Command Prompt, Terminal, or equivalent program on your device.
4. Navigate to the "rustman" folder (using `cd` on Windows).
5. Type `cargo run` to start the program.

### Using Visual Studio Code
1. If not already installed, [download and install the most recent version of Rust](https://www.rust-lang.org/tools/install). This should install all the necessary components (including Cargo).
2. Download the repository contents to your computer. You can either download the repository contents as a .ZIP file, or use git / GitHub Desktop to copy the repository to your device.
3. Open Visual Studio Code, and open the folder containing the project ("rustman").
4. Open a terminal (Terminal > New Terminal) and type `cargo run` in the terminal. The program will run using the terminal built into Visual Studio Code.

Instructions for using the software:

* Once the program has started, use your keyboard to input numbers to select your desired option. 
* Typing in `1` on the main menu will start the main game. Once a game has started, the program will display a hidden word represented by blank spaces. Try guessing the word hidden by the blanks by typing in alphabetical characters one at a time. Once you guess the entire word or have too many missed guesses, the game will end and tell you what the word is alongside the amount of guesses you took (as well as whether or not you won).
* Typing in `2` on the main menu will open the word list options, showing you the currently selected word list and the words within it. Typing `1` will allow you to select which word list the program will pull from in the main game. You can type `2` to create your own word list for use in the game as well. Typing in `0` will return you to the main menu.
* Typing in `3` on the main menu will allow you to edit the number of allowed missed guesses before the game ends. Numbers between 1-25 will be allowed here, so you can customize how easy or hard you want the game to be.
* Typing in `0` on the main menu will quit the program.

## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* [Visual Studio Code](https://code.visualstudio.com/)
* [rustup - Rust Installer (64-bit)](https://www.rust-lang.org/tools/install)
* [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
* [rust-analyzer - VSCode Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Rust Tutorial - W3Schools](https://www.w3schools.com/rust/index.php)
* [Rust Programming Language - GeeksforGeeks](https://www.geeksforgeeks.org/introduction-to-rust-programming-language/)
* [The Rust Programming Language Book](https://doc.rust-lang.org/book/title-page.html)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Add mid-game quit option
* [ ] Add the ability to delete word lists
* [ ] Save/load functionality for saving word lists & current lives