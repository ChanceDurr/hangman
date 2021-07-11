// Hopefully a working hangman Rust CLI
use rand::Rng;
use text_io::read;
use colorful::{Color, Colorful};

// Function to replace "underscored" word with correctly guessed letters
fn add_letters(end_word: &String, mut cur_word: String, letter: &String) -> String {
    let char_letter = letter.chars().next().expect("Should be just one char whats happening?");
    for (index, character) in end_word.char_indices() {
        if character == char_letter { cur_word.replace_range(index*2..(index*2)+1, letter.as_ref()) }
    }

    return cur_word
}


// Main function
fn main() {

    // Words with categories to choose from
    let words = [ ("orange", "Food"), ("boat", "Transportation"), ("thelegr", "Viewer of el Chance"), ("mountain dew", "Drink"), ("vase", "household item"), ("cemetery", "place"), ("lawyer", "occupation"), ("secretary", "occupation"), ("berry", "food"), ("engine", "piece of machinery"), ("vessel", "transportation") ];

    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");

    // Randomly choose word/hint tuple from words list
    let (word, hint) =  words[rand::thread_rng().gen_range(0..words.len())];

    // Create String from word
    let word = String::from(word);

    // Create mut var that is underscores of same length of the word
    let mut cur_guess = std::iter::repeat("_ ").take(word.len()).collect::<String>();
    
    // Variable in same format as cur_guess for checking if finished.
    let word_space: String = word.chars().map(|c| format!("{} ", c)).collect();

    // Instantiate guessed word variable for if word is guess correctly
    let mut guessed_word = false;

    // Keep count of incorrect guesses
    let mut incorrect_guesses = 0;

    // Keep track of already guessed letters
    let mut already_guessed = Vec::new();

    // Error variable
    let mut error = String::new();

    // Message variable for saying whether or not the guess was correct
    let mut message = String::new();

    // While loop for the game
    while guessed_word == false && incorrect_guesses < 6 {

        // Clear the screen for each loop
        print!("\x1B[2J\x1B[1;1H");

        let ascii_art = r#"
 ('-. .-.   ('-.         .-') _            _   .-')      ('-.         .-') _  
( OO )  /  ( OO ).-.    ( OO ) )          ( '.( OO )_   ( OO ).-.    ( OO ) ) 
,--. ,--.  / . --. /,--./ ,--,'  ,----.    ,--.   ,--.) / . --. /,--./ ,--,'  
|  | |  |  | \-.  \ |   \ |  |\ '  .-./-') |   `.'   |  | \-.  \ |   \ |  |\  
|   .|  |.-'-'  |  ||    \|  | )|  |_( O- )|         |.-'-'  |  ||    \|  | ) 
|       | \| |_.'  ||  .     |/ |  | .--, \|  |'.'|  | \| |_.'  ||  .     |/  
|  .-.  |  |  .-.  ||  |\    | (|  | '. (_/|  |   |  |  |  .-.  ||  |\    |   
|  | |  |  |  | |  ||  | \   |  |  '--'  | |  |   |  |  |  | |  ||  | \   |   
`--' `--'  `--' `--'`--'  `--'   `------'  `--'   `--'  `--' `--'`--'  `--'"#.trim_matches('\n');

        println!("{}", ascii_art.gradient(Color::Green).bold());


        // Print main message each loop
        println!("\
            Welcome to Hangman, brought to you by Rust.\
            \n\
            \n\
            Word: {cur_guess}\
            \n\
            Hint: {hint}\
            \n\
            Guessed Letters: {already_guessed:?}\
            \n", cur_guess=cur_guess, hint=hint, already_guessed = already_guessed);   

        // Print error message if exists
        if error.is_empty() == false { println!("{}\n", error.clone().gradient(Color::Red).bold()); error.clear(); };

        // Print message for correct/incorrect
        if message.is_empty() == false { println!("{}\n", message); message.clear() }; 

        println!("Please enter one letter:\n");

        // Get the input and save it to the line variable
        let mut guess: String = read!("{}\n"); 

        // Format guess to lowercase
        guess = guess.to_lowercase();
       
        // Check if input is valid
        if guess.len() > 1 {
            error = String::from("Too many characters");
            continue;
        }

        // Check if letter already guessed
        if already_guessed.contains(&guess) {
            error = String::from("Letter already guessed");
            continue;
        }
        
        // Check if the letter was a correct letter
        if word.contains(&guess) {
            message = String::from("That is a correct letter");
            cur_guess = add_letters(&word, cur_guess, &guess); 
            already_guessed.push(guess);
        } else {
            message = String::from("That letter isn't in the word");
            incorrect_guesses += 1;
            already_guessed.push(guess);
        }
        
        // Check to see if we have guessed the word
        if cur_guess == word_space { guessed_word = true };
    }

    // Winning message
    if guessed_word == true { println!("You have won! The correct word was {}", word.gradient(Color::Green).bold()) };

    // Losing Message 
    if incorrect_guesses == 6 { println!("The stickman has been hung/hanged/hunged?") };
}









