#[cfg(doc)]
use super::Game;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Letter {
    pub letter: char,
    pub position: Position,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    NotInWord,
    WrongPosition,
    Correct,
}

impl Letter {
    /// Create a new [Letter] struct.
    ///
    /// This constructor will automatically convert the letter to uppercase.
    pub fn new(letter: char, position: Position) -> Self {
        Self {
            letter: letter.to_ascii_uppercase(),
            position,
        }
    }

    /// Check the pair of letters against the expected word.
    ///
    /// Return `Some(Letter)` if the position can be known, or [None] if the position is more
    /// complex. When we return [None], that means that the position of the letter is either
    /// [WrongPosition](Position::WrongPosition) or [NotInWord](Position::NotInWord), but we don't
    /// know enough context to figure it out.
    ///
    /// The context we need is the target word and the rest of the guess, and the logic for working
    /// it out is in [make_guess](Game::make_guess).
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
