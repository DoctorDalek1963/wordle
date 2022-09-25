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
fn pretty_print_letter(letter: Letter) -> String {
    let mut string: String = match letter.position {
        Position::NotInWord => {
            format!("{}", color::Fg(color::Black))
        }
        Position::WrongPosition => {
            format!("{}", color::Fg(color::Yellow))
        }
        Position::Correct => {
            format!("{}", color::Fg(color::Green))
        }
    };

    string.push(letter.letter);
    string
}

fn main() {
    let game = Game::new();

    let validator = |input: &str| {
        let valid = Game::is_valid_guess(input);
        match valid {
            Ok(()) => Ok(Validation::Valid),
            Err(error) => Ok(Validation::Invalid(error.into())),
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
                let letters = game.make_guess(&guess).expect(
                    "User should not have been able to enter any invalid guess: `{guess:?}`",
                );

                print!("{}", style::Bold);
                for letter in letters.map(|l| pretty_print_letter(l)) {
                    print!("{}", letter);
                }
                println!("{}", style::Reset);

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
