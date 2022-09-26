use std::collections::HashMap;

use inquire::{
    ui::{RenderConfig, Styled},
    validator::Validation,
    Text,
};
use termion::style;
use wordle::{
    letters::{Letter, Position},
    Game,
};

const TOTAL_GUESSES: u8 = 6;

/// Return a string with the given letter and the appropriate colour for its position type.
///
/// The colours are based on the original Wordle game, and implemented using Termion.
///
/// Ideally, the word should also be printed in bold. This is left up to the caller, as this
/// function only handles individual letters. Additionally, this function DOES NOT RESET the
/// terminal colours at the end of the letter. Each colour overrides the last, and the colours
/// only need to be reset at the end of the word.
fn pretty_print_letter_with_position(letter: &char, position: &Option<Position>) -> String {
    use termion::color;

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
/// See [`pretty_print_letter_with_position`].
fn pretty_print_letter_struct(letter: &Letter) -> String {
    pretty_print_letter_with_position(&letter.letter, &Some(letter.position))
}

/// Print the player's guess word highlighted according to classic Wordle colours, indented by 7 spaces.
///
/// The identation is to align with the printed keyboard. See [`print_keyboard`].
fn print_guess(letters: &[Letter; 5]) {
    print!("       {}", style::Bold);
    for letter in letters.map(|l| pretty_print_letter_struct(&l)) {
        print!("{}", letter);
    }
    println!("{}", style::Reset);
}

/// Print the standard QWERTY keyboard with the letters highlighted as the best position they've
/// seen in a previous guess.
///
/// See [`Game::keyboard`].
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

/// Clear the terminal.
///
/// This CLI binary is only designed for Linux terminals, so we use ANSI codes.
fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn create_render_config(guesses: &u8) -> RenderConfig {
    use inquire::ui::Color;

    // This section is needed because RenderConfig.prompt_prefix needs to be
    // Styled<&'static str>, so the string needs to be a literal

    // NOTE: If we ever change the TOTAL_GUESSES, then this section also needs to change

    let prompt_prefix = Styled::new(match guesses {
        6 => "(1/6) >",
        5 => "(2/6) >",
        4 => "(3/6) >",
        3 => "(4/6) >",
        2 => "(5/6) >",
        1 => "(6/6) >",
        _ => unreachable!("We should never want a prompt with more than 6 guesses"),
    })
    .with_fg(Color::LightGreen);

    let answered_prompt_prefix = Styled::new(match guesses {
        6 => "(1/6) >",
        5 => "(2/6) >",
        4 => "(3/6) >",
        3 => "(4/6) >",
        2 => "(5/6) >",
        1 => "(6/6) >",
        _ => unreachable!("We should never want a prompt with more than 6 guesses"),
    })
    .with_fg(Color::Black);

    let mut config = RenderConfig::default_colored();
    config.prompt_prefix = prompt_prefix;
    config.answered_prompt_prefix = answered_prompt_prefix;

    config
}

fn main() {
    let mut game = Game::new();

    let validator = |input: &str| {
        let valid = Game::is_valid_guess(input);
        match valid {
            Ok(()) => Ok(Validation::Valid),
            Err((error, _)) => Ok(Validation::Invalid(error.into())),
        }
    };

    let mut remaining_guesses: u8 = TOTAL_GUESSES;
    let mut past_guesses: Vec<[Letter; 5]> = Vec::new();

    println!("Welcome to Wordle!\n");

    loop {
        if remaining_guesses == 0 {
            println!("\nOut of guesses!");
            println!("Thanks for playing Wordle! The word was {}!", game.word);
            break;
        };

        match Text::new("")
            .with_render_config(create_render_config(&remaining_guesses))
            .with_validator(validator)
            .with_formatter(&str::to_ascii_uppercase)
            .prompt()
        {
            Ok(guess) => {
                let letters = game.make_guess(&guess).unwrap_or_else(|_| {
                    panic!("User should not have been able to enter any invalid guess: {guess:?}")
                });

                past_guesses.push(letters);

                clear_terminal();

                for guess in &past_guesses {
                    print_guess(guess);
                }
                println!();

                print_keyboard(&game.keyboard);

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
                println!("\nThanks for playing Wordle! The word was {}!", game.word);
                break;
            }
        };
    }
}
