//! # Wordle
//!
//! A library to handle the backend details of standard Wordle games.
//! See [the New York Times' Wordle](https://www.nytimes.com/games/wordle/index.html).

pub mod letters;
pub mod valid_words;

pub mod prelude {
    //! This module just re-exports some commonly used types.

    pub use super::letters::{Letter, Position};
    pub use super::{Game, GuessError, Word};
}

use letters::{Letter, Position};
use rand::seq::SliceRandom;
use std::{cmp::Ordering, collections::HashMap};
use thiserror::Error;

/// A word is just an array of 5 [`Letter`]s.
pub type Word = [Letter; 5];

/// An enum representing possible errors resulting from an invalid guess.
#[derive(Debug, Error, PartialEq)]
pub enum GuessError {
    /// The guess must be exclusively ASCII characters.
    ///
    /// This is just because the word list is exclusively ASCII characters.
    #[error("Guess must be exclusively ASCII characters")]
    IncludesNonAscii,

    /// The guess must be in the [`VALID_WORDS`](valid_words::VALID_WORDS) list.
    #[error("Guess must be a valid word")]
    InvalidWord,

    /// The guess must be exactly 5 letters.
    #[error("Guess must be exactly 5 letters")]
    WrongWordLength,
}

/// A game of Wordle.
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    /// The target word that the user needs to guess.
    pub word: String,

    /// This hashmap contains all uppercase Latin letters, and maps them to the best
    /// position that they've been seen in previously.
    ///
    /// If they have not been guessed previously, this is [`None`], otherwise
    /// [`NotInWord`](Position::NotInWord) is the lowest position, then
    /// [`WrongPosition`](Position::WrongPosition), and then [`Correct`](Position::Correct).
    pub keyboard: HashMap<char, Option<Position>>,
}

impl Game {
    /// Create a game by choosing a random target word from [`GOOD_WORDS`](valid_words::GOOD_WORDS).
    ///
    /// This constructor also ensures that the [`keyboard`](Game::keyboard) contains all uppercase
    /// Latin letters, and initially maps them all to [`None`]. See
    /// [`new_keyboard_map`](Game::new_keyboard_map).
    pub fn new() -> Self {
        Self {
            word: {
                let word = *valid_words::GOOD_WORDS
                    .choose(&mut rand::thread_rng())
                    .expect("valid_words::GOOD_WORDS should never be empty");
                word.to_string().to_ascii_uppercase()
            },
            keyboard: Self::new_keyboard_map(),
        }
    }

    /// Create an empty keyboard map.
    pub fn new_keyboard_map() -> HashMap<char, Option<Position>> {
        let mut map = HashMap::new();
        for c in valid_words::ALPHABET {
            map.insert(c, None);
        }
        map
    }

    /// Check if the guess is valid, returning `Ok(())` if it is.
    ///
    /// A guess is only valid if it is exclusively ASCII, 5 characters long, and be in the list.
    ///
    /// A guess does not have to be uppercase to be valid. It is made uppercase automatically.
    ///
    /// # Errors
    ///
    /// If a guess is invalid, then we return the appropriate [`GuessError`] variant.
    pub fn is_valid_guess(guess: &str) -> Result<(), GuessError> {
        let guess = guess.to_ascii_uppercase();

        if !guess.is_ascii() {
            return Err(GuessError::IncludesNonAscii);
        } else if guess.len() != 5 {
            return Err(GuessError::WrongWordLength);
        } else if !valid_words::VALID_WORDS.contains(&&guess[..]) {
            return Err(GuessError::InvalidWord);
        }

        Ok(())
    }

    /// Guess the given word against the target word.
    ///
    /// This method returns an array of five [`Letter`](letters::Letter)s. Each Letter has a [`Position`](letters::Position).
    /// As per classic Wordle rules, the positions are calculated as follows:
    ///
    /// If a letter is in the word and in the correct position, then it is [`Correct`](letters::Position::Correct).
    /// If a letter is not in the word at all, then it is [`NotInWord`](letters::Position::NotInWord).
    ///
    /// If a letter is in the word but not in the correct position, then:
    /// If there are more occurences of that letter in the target word, it is in the [`WrongPosition`](letters::Position::WrongPosition).
    /// If all the occurences of that letter have been placed correctly, or already accounted for
    /// by [`WrongPosition`](letters::Position::WrongPosition) letters, then it is
    /// [`NotInWord`](letters::Position::NotInWord).
    ///
    /// # Errors
    ///
    /// If the guess is invalid, we return the appropriate [`GuessError`] variant. See
    /// [`is_valid_guess`](Game::is_valid_guess).
    pub fn make_guess(&mut self, guess: &str) -> Result<Word, GuessError> {
        Self::is_valid_guess(guess)?;

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
        // correctly placed letters in the guess, and the second number is how many times that
        // letter still needs to be placed in the guess
        let mut correct_letters_map: HashMap<char, (usize, usize)> = HashMap::new();
        for c in valid_words::ALPHABET {
            let correct_letters = optional_letters
                .iter()
                .filter(|l| match l.1 {
                    None => false,
                    Some(ll) => ll.letter == c && ll.position == Position::Correct,
                })
                .count();
            correct_letters_map.insert(c, (correct_letters, instances_in_word_map.get(&c).expect("`instances_in_word_map` should contain all letters in the Latin alphabet ({c:?})") - correct_letters));
        }

        let word: Word = optional_letters.map(|(orig_char, opt_letter)|
            opt_letter.map_or_else(|| {
                // If we get here, then the letter is either in the wrong position, or all
                // occurences of this letter have been placed correctly already
                let instances_in_word = instances_in_word_map.get(&orig_char).expect("`instances_in_word_map` should contain all letters in the Latin alphabet ({orig_char:?})");

                let (instances_in_correct_positions_in_guess, remaining_places): &(usize, usize) =
                    correct_letters_map.get(&orig_char).expect(
                        "`correct_letters_map` should contain all letters in the Latin alphabet ({orig_char:?})",
                    );

                // We know how many times this letter appears in the word and in correct positions
                // in the current guess
                // We also know that this letter is not in the correct position, and instances_in_word > 0

                match instances_in_word.cmp(instances_in_correct_positions_in_guess) {
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
            }, |l| l)
        );

        self.update_keyboard(&word);

        Ok(word)
    }

    /// Update the game's keyboard according to the positions of the letters in the given guess.
    fn update_keyboard(&mut self, letters: &Word) {
        use ordered_position::OrderedPosition;

        for letter in letters {
            let current_pos = self
                .keyboard
                .get(&letter.letter)
                .expect("Game::keyboard should contain all Latin letters");

            if OrderedPosition(Some(letter.position)).cmp(&OrderedPosition(*current_pos))
                == Ordering::Greater
            {
                let pos = self
                    .keyboard
                    .get_mut(&letter.letter)
                    .expect("Game::keyboard should contain all Latin letters");
                *pos = Some(letter.position);
            }
        }
    }
}

mod ordered_position {
    //! This module is an implementation detail to allow the [`Game::update_keyboard`] method to
    //! correctly order the `Option<Position>` types.

    use super::*;

    /// This struct is a thin wrapper around `Option<Position>` and allows a strict ordering of
    /// this type.
    ///
    /// All variants are equal to themselves. `None` is less than everything else, then
    /// [`NotInWord`](letters::Position::NotInWord), then
    /// [`WrongPosition`](letters::Position::WrongPosition), and finally
    /// [`Correct`](letters::Position::Correct) is greater than everything else.
    #[derive(Debug, Eq, PartialEq)]
    pub struct OrderedPosition(pub Option<Position>);

    impl PartialOrd<Self> for OrderedPosition {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let this = self.0;
            let other = other.0;

            Some(match this {
                None => match other {
                    None => Ordering::Equal,
                    _ => Ordering::Less,
                },
                Some(pos) => match pos {
                    Position::NotInWord => match other {
                        None => Ordering::Greater,
                        Some(Position::NotInWord) => Ordering::Equal,
                        Some(Position::WrongPosition | Position::Correct) => Ordering::Less,
                    },
                    Position::WrongPosition => match other {
                        None | Some(Position::NotInWord) => Ordering::Greater,
                        Some(Position::WrongPosition) => Ordering::Equal,
                        Some(Position::Correct) => Ordering::Less,
                    },
                    Position::Correct => match other {
                        Some(Position::Correct) => Ordering::Equal,
                        _ => Ordering::Greater,
                    },
                },
            })
        }
    }

    impl Ord for OrderedPosition {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other)
                .expect("Comparing two `OrderedPosition` structs should never return `None`")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_guess_invalid_inputs() {
        let mut game = Game::new();

        for guess in ["spurg", "HYiiA", "olleh"] {
            assert_eq!(game.make_guess(guess), Err(GuessError::InvalidWord));
            assert_eq!(Game::is_valid_guess(guess), Err(GuessError::InvalidWord));
        }

        for guess in ["Öster", "Złoty", "Schrödinger"] {
            assert_eq!(game.make_guess(guess), Err(GuessError::IncludesNonAscii));
            assert_eq!(
                Game::is_valid_guess(guess),
                Err(GuessError::IncludesNonAscii)
            );
        }

        for guess in ["", "hi", "this should fail"] {
            assert_eq!(game.make_guess(guess), Err(GuessError::WrongWordLength));
            assert_eq!(
                Game::is_valid_guess(guess),
                Err(GuessError::WrongWordLength)
            );
        }
    }

    #[test]
    fn make_guess_correct_output() {
        let mut game = Game {
            word: "DYSON".to_string(),
            keyboard: Game::new_keyboard_map(),
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

        let mut game = Game {
            word: "BLEEP".to_string(),
            keyboard: Game::new_keyboard_map(),
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

        let mut game = Game {
            word: "EERIE".to_string(),
            keyboard: Game::new_keyboard_map(),
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

    #[test]
    fn ordered_position() {
        use ordered_position::OrderedPosition;

        let n = OrderedPosition(None);
        let niw = OrderedPosition(Some(Position::NotInWord));
        let wp = OrderedPosition(Some(Position::WrongPosition));
        let c = OrderedPosition(Some(Position::Correct));

        assert!(n == n);
        assert!(n < niw);
        assert!(n < wp);
        assert!(n < c);

        assert!(niw > n);
        assert!(niw == niw);
        assert!(niw < wp);
        assert!(niw < c);

        assert!(wp > n);
        assert!(wp > niw);
        assert!(wp == wp);
        assert!(wp < c);

        assert!(c > n);
        assert!(c > niw);
        assert!(c > wp);
        assert!(c == c);
    }
}
