pub mod hangman {

    //Filters repeated guesses.
    pub fn is_valid(guess: &char, guesses: &Vec<char>) -> bool{
        if guesses.contains(guess) {
            return false;
        }
        true
    }

    pub fn contains_guess(guess: &char, word: &str) -> bool{
        for c in word.chars(){
            if c == *guess {
                return true;
            }
        }
        false
    }

}