use std::io::stdin;
use std::collections::hash_map::HashMap;
use lib::hangman;

mod lib;

//Returns guess as a char
//TODO: Allow to guess whole word
fn get_input() -> char{

    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Invalid input");

    guess.trim().chars().nth(0).expect("Error converting to character")
}

//Change return to tuple / struct
fn find_char_pos(guess: &char, word: &str) -> HashMap<char, Vec<usize>> {

    let mut temp_map: HashMap<char, Vec<usize>> = HashMap::new();

    for (num, c) in word.char_indices() {

        if c == *guess {

            if !temp_map.contains_key(&c) {
                temp_map.insert(c, vec![num]);
            } else {
               temp_map.get_mut(&c).unwrap().push(num);
            }

        }

    }

    temp_map

}

fn matches_word(word_progress: &[char], word: &str) -> bool{
    let mut temp_s = String::new();
    
    for x in word_progress.iter() {
        temp_s.push(*x);
    }
    //debug
    println!("{} == {}", temp_s, word);
    if temp_s == word {
        return true;
    }
    false
}

fn main() {

    const WORD: &str = "alabama";

    let mut guesses: Vec<char> = Vec::new();
    let mut word_progress: [char; WORD.len()] = ['_'; WORD.len()];

    loop {

        println!("Guess a letter: ");        
        let input = get_input();

        //Is the guess valid?
        if hangman::is_valid(&input, &guesses) {
            guesses.push(input);

            if hangman::contains_guess(&input, &WORD) {
                let temp_map = find_char_pos(&input, &WORD);
                
                for vecs in temp_map.values(){
                    for num in vecs {
                        word_progress[*num] = input;
                    }
                }
                if matches_word(&word_progress, &WORD) {
                    println!("You win! The word was: {}", WORD);
                    break;
                }
                println!("{:?}", word_progress);
            }

        }
            
                //Find character positions, used positions to place chars into new word(vec) 
        
    }
}
