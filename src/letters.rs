//! This module handles the concept of letters and their associated positions.

/// A letter with an associated [`Position`] in the word.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Letter {
    /// The actual character that this Letter wraps.
    pub letter: char,

    /// The position of this letter in the word.
    pub position: Position,
}

/// A position in the word.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    /// The letter doesn't appear in the word at all, or all the instances of that letter have
    /// already been placed in the word.
    NotInWord,

    /// The letter appears in the word, but not in this position.
    WrongPosition,

    /// The letter appears in the word in this position.
    ///
    /// It may also appear elsewhere in the word.
    Correct,
}

impl Letter {
    /// Create a new letter with the given associated position.
    ///
    /// This constructor will automatically convert the letter character to uppercase.
    pub fn new(letter: char, position: Position) -> Self {
        Self {
            letter: letter.to_ascii_uppercase(),
            position,
        }
    }

    /// Check the pair of letters against the expected word.
    ///
    /// Return `Some(Letter)` if the position can be known, or [`None`] if the position is more
    /// complex. When we return [`None`], that means that the position of the letter is either
    /// [`WrongPosition`](Position::WrongPosition) or [`NotInWord`](Position::NotInWord), but we don't
    /// know enough context to figure it out.
    ///
    /// The context we need is the target word and the rest of the guess, and the logic for working
    /// it out is in [`Game::make_guess`](super::Game::make_guess).
    pub fn simple_check_letter_pair(
        letter: &char,
        expected_letter: &char,
        word: &str,
    ) -> Option<Self> {
        let position = if *letter == *expected_letter {
            Position::Correct
        } else if !word.contains(*letter) {
            Position::NotInWord
        } else {
            return None;
        };

        Some(Self {
            letter: *letter,
            position,
        })
    }
}
