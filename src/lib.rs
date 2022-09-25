pub mod letters;
pub mod valid_words;

use letters::{Letter, Position};
use rand::seq::SliceRandom;
use std::{cmp::Ordering, collections::HashMap};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum WordleError {
    #[error("Guess must be exactly 5 letters")]
    WrongWordLength,

    #[error("Guess must be exclusively ASCII characters")]
    IncludesNonAscii,

    #[error("Guess must be a valid word")]
    InvalidWord,
}

pub struct Game {
    pub word: String,
}

impl Game {
    /// Create a game by choosing a random word from [GOOD_WORDS](valid_words::GOOD_WORDS).
    pub fn new() -> Self {
        let word = *valid_words::GOOD_WORDS
            .choose(&mut rand::thread_rng())
            .expect("valid_words::GOOD_WORDS should never be empty");
        Self {
            word: word.to_string().to_ascii_uppercase(),
        }
    }

    /// Check if the guess is valid.
    ///
    /// A guess is only valid if it is a five letter word in [VALID_WORDS](valid_words::VALID_WORDS).
    /// That means it must be exclusively ASCII, have a length of 5 characters, and be in the list.
    ///
    /// A guess does not have to be uppercase to be valid. It is made uppercase automatically.
    ///
    /// If a guess is invalid, we return the appropriate [WordleError] variant.
    pub fn is_valid_guess(guess: &str) -> Result<(), WordleError> {
        let guess = guess.to_ascii_uppercase();

        if !guess.is_ascii() {
            return Err(WordleError::IncludesNonAscii);
        } else if guess.len() != 5 {
            return Err(WordleError::WrongWordLength);
        } else if !valid_words::VALID_WORDS.contains(&&guess[..]) {
            return Err(WordleError::InvalidWord);
        }

        Ok(())
    }

    /// Guess the given word against the secret word.
    ///
    /// This method returns an array of five [Letter](letters::Letter)s. Each Letter has a [Position](letters::Position).
    /// As per classic Wordle rules, the positions are calculated as follows:
    ///
    /// If a letter is in the word and in the correct position, then it is [Correct](letters::Position::Correct).
    /// If a letter is not in the word at all, then it is [NotInWord](letters::Position::NotInWord).
    ///
    /// If a letter is in the word but not in the correct position, then:
    /// If there are more occurences of that letter in the target word, it is in the [WrongPosition](letters::Position::WrongPosition).
    /// If all the occurences of that letter have been placed correctly, it is [NotInWord](letters::Position::NotInWord).
    pub fn make_guess(&self, guess: &str) -> Result<[Letter; 5], WordleError> {
        Game::is_valid_guess(&guess)?;

        let guess = guess.to_ascii_uppercase();

        let pairs: Vec<(char, char)> = guess.chars().zip(self.word.chars()).collect();

        let optional_letters: [(char, Option<Letter>); 5] = [
            (
                pairs[0].0,
                Letter::simple_check_letter_pair(&pairs[0].0, &pairs[0].1, &self.word),
            ),
            (
                pairs[1].0,
                Letter::simple_check_letter_pair(&pairs[1].0, &pairs[1].1, &self.word),
            ),
            (
                pairs[2].0,
                Letter::simple_check_letter_pair(&pairs[2].0, &pairs[2].1, &self.word),
            ),
            (
                pairs[3].0,
                Letter::simple_check_letter_pair(&pairs[3].0, &pairs[3].1, &self.word),
            ),
            (
                pairs[4].0,
                Letter::simple_check_letter_pair(&pairs[4].0, &pairs[4].1, &self.word),
            ),
        ];

        // This maps each letter to its number of occurences in the target word
        let mut instances_in_word_map: HashMap<char, usize> = HashMap::new();
        for c in valid_words::ALPHABET {
            instances_in_word_map.insert(c, self.word.chars().filter(|cc| *cc == c).count());
        }

        // Shadow to make it immutable
        let instances_in_word_map = instances_in_word_map;

        // This maps each character in the alphabet to a tuple. The first element is the number of
        // correctly places letters in the guess, and the second number is how many times that
        // letter still needs to be placed in the guess
        let mut correct_letters_map: HashMap<char, (usize, usize)> = HashMap::new();
        for c in valid_words::ALPHABET {
            let correct_letters = optional_letters
                .iter()
                .filter(|l| {
                    l.1.is_some()
                        && l.1.as_ref().unwrap().letter == c
                        && l.1.as_ref().unwrap().position == Position::Correct
                })
                .count();
            correct_letters_map.insert(c, (correct_letters, instances_in_word_map.get(&c).expect("`instances_in_word_map` should contain all letters in the Latin alphabet ({c:?})") - correct_letters));
        }

        let letters: [Letter; 5] = optional_letters.map(|(orig_char, opt_letter)| {
            if opt_letter.is_some() {
                opt_letter.unwrap()
            } else {
                // If we get here, then the letter is either in the wrong position, or all
                // occurences of this letter have been placed correctly already
                let instances_in_word = instances_in_word_map.get(&orig_char).expect("`instances_in_word_map` should contain all letters in the Latin alphabet ({orig_char:?})");

                let (instances_in_correct_positions_in_guess, remaining_places) =
                    correct_letters_map.get(&orig_char).expect(
                        "`correct_letters_map` should contain all letters in the Latin alphabet ({orig_char:?})",
                    );

                // We know how many times this letter appears in the word and in correct positions
                // in the current guess
                // We also know that this letter is not in the correct position, and instances_in_word > 0

                match instances_in_word.cmp(&instances_in_correct_positions_in_guess) {
                    Ordering::Greater => {
                        if *remaining_places > 0 {
                            // The letter needs to stay in the guess, but in a different position
                            // We also want to decrement the remaining uses of this letter
                            correct_letters_map
                                .get_mut(&orig_char)
                                .expect("`correct_letters_map` should contain all letters in the Latin alphabet ({orig_char:?})")
                                .1 -= 1;
                            Letter::new(orig_char, Position::WrongPosition)
                        } else {
                            // We've used up all the remaining places for this character
                            Letter::new(orig_char, Position::NotInWord)
                        }
                    }
                    Ordering::Equal => {
                        // We already have enough instances of this letter
                        Letter::new(orig_char, Position::NotInWord)
                    }
                    Ordering::Less => unreachable!(concat!(
                        "We cannot have more instances of the letter in the correct position ",
                        "in the guess than there are instances in the target word"
                    )),
                }
            }
        });

        Ok(letters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_guess_invalid_inputs() {
        let game = Game::new();

        for guess in ["spurg", "HYiiA", "olleh"] {
            assert_eq!(game.make_guess(guess), Err(WordleError::InvalidWord));
            assert_eq!(Game::is_valid_guess(guess), Err(WordleError::InvalidWord));
        }

        for guess in ["Öster", "Złoty", "Schrödinger"] {
            assert_eq!(game.make_guess(guess), Err(WordleError::IncludesNonAscii));
            assert_eq!(
                Game::is_valid_guess(guess),
                Err(WordleError::IncludesNonAscii)
            );
        }

        for guess in ["", "hi", "this should fail"] {
            assert_eq!(game.make_guess(guess), Err(WordleError::WrongWordLength));
            assert_eq!(
                Game::is_valid_guess(guess),
                Err(WordleError::WrongWordLength)
            );
        }
    }

    #[test]
    fn make_guess_correct_output() {
        let game = Game {
            word: "DYSON".to_string(),
        };

        assert_eq!(
            game.make_guess("WORDY")
                .expect("input `WORDY` should be a valid guess"),
            [
                Letter::new('w', Position::NotInWord),
                Letter::new('o', Position::WrongPosition),
                Letter::new('r', Position::NotInWord),
                Letter::new('d', Position::WrongPosition),
                Letter::new('y', Position::WrongPosition),
            ]
        );
        assert_eq!(
            game.make_guess("DADDY")
                .expect("input `DADDY` should be a valid guess"),
            [
                Letter::new('d', Position::Correct),
                Letter::new('a', Position::NotInWord),
                // Although there's a 'D' at the start, that's already been counted,
                // so this second and third 'D' should be NotInWord
                Letter::new('d', Position::NotInWord),
                Letter::new('d', Position::NotInWord),
                Letter::new('y', Position::WrongPosition),
            ]
        );
        assert_eq!(
            game.make_guess("dySOn")
                .expect("input `dySOn` should be a valid guess"),
            [
                Letter::new('D', Position::Correct),
                Letter::new('Y', Position::Correct),
                Letter::new('s', Position::Correct),
                Letter::new('o', Position::Correct),
                Letter::new('N', Position::Correct),
            ]
        );
        assert_eq!(
            game.make_guess("HySoN")
                .expect("input `HySoN` should be a valid guess"),
            [
                Letter::new('h', Position::NotInWord),
                Letter::new('Y', Position::Correct),
                Letter::new('s', Position::Correct),
                Letter::new('O', Position::Correct),
                Letter::new('n', Position::Correct),
            ]
        );
        assert_eq!(
            game.make_guess("sassy")
                .expect("input `sassy` should be a valid guess"),
            [
                // The 'S' in the middle is Correct, and it's the only 'S',
                // so the other two should be NotInWord
                Letter::new('s', Position::NotInWord),
                Letter::new('a', Position::NotInWord),
                Letter::new('s', Position::Correct),
                Letter::new('s', Position::NotInWord),
                Letter::new('y', Position::WrongPosition),
            ]
        );
        assert_eq!(
            game.make_guess("dusty")
                .expect("input `dusty` should be a valid guess"),
            [
                Letter::new('d', Position::Correct),
                Letter::new('u', Position::NotInWord),
                Letter::new('s', Position::Correct),
                Letter::new('t', Position::NotInWord),
                Letter::new('y', Position::WrongPosition),
            ]
        );

        let game = Game {
            word: "BLEEP".to_string(),
        };

        assert_eq!(
            game.make_guess("eerie")
                .expect("input `eerie` should be a valid guess"),
            [
                // Only the first 2 'E's should be WrongPosition, because there's only 2 unplaced 'E's in the word
                Letter::new('e', Position::WrongPosition),
                Letter::new('e', Position::WrongPosition),
                Letter::new('r', Position::NotInWord),
                Letter::new('i', Position::NotInWord),
                Letter::new('e', Position::NotInWord),
            ]
        );

        let game = Game {
            word: "EERIE".to_string(),
        };

        assert_eq!(
            game.make_guess("bleep")
                .expect("input `bleep` should be a valid guess"),
            [
                Letter::new('b', Position::NotInWord),
                Letter::new('l', Position::NotInWord),
                Letter::new('e', Position::WrongPosition),
                Letter::new('e', Position::WrongPosition),
                Letter::new('p', Position::NotInWord),
            ]
        )
    }
}