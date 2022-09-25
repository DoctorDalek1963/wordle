use std::collections::HashMap;

use inquire::{validator::Validation, Text};
use termion::{color, style};
use wordle::{
    letters::{Letter, Position},
    Game,
};

/// Return a string with the given letter and the appropriate colour for its position type.
///
/// The colours are based on the original Wordle game, and implemented using Termion.
///
/// Ideally, the word should also be printed in bold. This is left up to the caller, as this
/// function only handles individual letters. Additionally, this function DOES NOT RESET the
/// terminal colours at the end of the letter. Each colour overrides the last, and the colours
/// only need to be reset at the end of the word.
fn pretty_print_letter_with_position(letter: &char, position: &Option<Position>) -> String {
    let mut string: String = match position {
        None => format!("{}", color::Fg(color::White)),
        Some(position) => match position {
            Position::NotInWord => {
                format!("{}", color::Fg(color::Black))
            }
            Position::WrongPosition => {
                format!("{}", color::Fg(color::Yellow))
            }
            Position::Correct => {
                format!("{}", color::Fg(color::Green))
            }
        },
    };

    string.push(*letter);
    string
}

/// Return a string with the given letter and the appropriate colour for its position type.
///
/// See [pretty_print_letter_with_position].
fn pretty_print_letter_struct(letter: &Letter) -> String {
    pretty_print_letter_with_position(&letter.letter, &Some(letter.position))
}

/// Print the player's guess word highlighted according to classic Wordle colours.
fn print_guess(letters: &[Letter; 5]) {
    print!("{}", style::Bold);
    for letter in letters.map(|l| pretty_print_letter_struct(&l)) {
        print!("{}", letter);
    }
    println!("{}", style::Reset);
}

/// Print the standard QWERTY keyboard with the letters highlighted as the best position they've
/// seen in a previous guess.
///
/// See [Game::keyboard].
fn print_keyboard(keyboard: &HashMap<char, Option<Position>>) {
    // We're assuming a standard QWERTY keyboard for convenience
    const ROW_1: [char; 10] = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'];
    const ROW_2: [char; 9] = ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'];
    const ROW_3: [char; 7] = ['Z', 'X', 'C', 'V', 'B', 'N', 'M'];

    macro_rules! print_row {
        ( $x:ident ) => {
            for letter in &$x {
                let position = keyboard
                    .get(letter)
                    .expect("Game::keyboard should contain all Latin letters");
                print!("{} ", pretty_print_letter_with_position(letter, position));
            }
        };
    }

    print!("{}", style::Bold);

    print_row!(ROW_1);
    println!();

    print!(" ");
    print_row!(ROW_2);
    println!();

    print!("  ");
    print_row!(ROW_3);

    println!("{}", style::Reset);
}

fn main() {
    let mut game = Game::new();

    let validator = |input: &str| {
        let valid = Game::is_valid_guess(input);
        match valid {
            Ok(()) => Ok(Validation::Valid),
            Err((error, guess)) => {
                // If the guess is the empty string, then we want to show the user the keyboard
                if guess.is_empty() {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(error.into()))
                }
            }
        }
    };

    let mut remaining_guesses: u8 = 6;

    println!("Welcome to Wordle!\n");

    loop {
        if remaining_guesses == 0 {
            println!("\nOut of guesses!");
            println!("Thanks for playing Worlde! The word was {}!", game.word);
            break;
        };

        match Text::new("")
            .with_validator(validator)
            .with_formatter(&|input: &str| input.to_ascii_uppercase())
            .prompt()
        {
            Ok(guess) => {
                if guess.is_empty() {
                    print_keyboard(&game.keyboard);

                    // This doesn't count as a guess, so we keep looping
                    continue;
                };

                let letters = game.make_guess(&guess).expect(
                    "User should not have been able to enter any invalid guess: `{guess:?}`",
                );

                print_guess(&letters);

                if letters
                    .iter()
                    .filter(|l| l.position == Position::Correct)
                    .count()
                    == 5
                {
                    println!("\nCongratulations! The word was {}!", game.word);
                    break;
                }

                remaining_guesses -= 1;
            }
            Err(_) => {
                println!("\nThanks for playing Worlde! The word was {}!", game.word);
                break;
            }
        };
    }
}
