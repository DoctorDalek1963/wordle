//! This module handles components for the game board itself - the 6 rows of 5 letter words.

use gloo_utils::window;
use js_sys::{Function, Promise};
use wordle::prelude::*;
use yew::{classes, function_component, html, Html, Properties};

/// Get the inner size of the window, returned as `Option<(width, height)>`.
fn get_window_size() -> Option<(i32, i32)> {
    let width = match window().inner_width() {
        Ok(val) => val.as_f64()? as i32,
        Err(_) => return None,
    };
    let height = match window().inner_height() {
        Ok(val) => val.as_f64()? as i32,
        Err(_) => return None,
    };

    Some((width, height))
}

#[doc(hidden)]
fn min(a: i32, b: i32) -> i32 {
    use std::cmp::Ordering;
    match a.cmp(&b) {
        Ordering::Less => a,
        _ => b,
    }
}

/// An enum to represent the state of a [`LetterComp`].
///
/// This is needed because each letter on the board can be blank, a [`Letter`] with a position, or
/// part of a guess currently being typed.
#[derive(Clone, PartialEq)]
enum LetterPropState {
    /// This letter is part of a previous guess, so it has an associated position.
    Concrete(Letter),

    /// This letter is part of a guess currently being typed, so it's just a character.
    CurrentGuess(char),

    /// This `LetterComp` is empty.
    Empty,
}

/// The props for [`LetterComp`].
#[derive(Clone, PartialEq, Properties)]
struct LetterProps {
    /// The delay used for the animation of revealing the letters.
    delay: u32,

    /// The state and contents of the component.
    letter: LetterPropState,
}

/// A component for a single letter in a row.
///
/// See [`LetterPropState`] for possible states.
#[function_component(LetterComp)]
fn letter_comp(props: &LetterProps) -> Html {
    fn position_to_class(letter: Letter) -> &'static str {
        match letter.position {
            Position::NotInWord => "notinword",
            Position::WrongPosition => "wrongposition",
            Position::Correct => "correct",
        }
    }

    match props.letter {
        LetterPropState::Empty => html! {
            <div class="letter empty" />
        },
        LetterPropState::Concrete(letter) => html! {
            <div class={classes!("letter", position_to_class(letter))} style={format!("animation-delay: {}ms;", props.delay)}>
                {letter.letter}
            </div>
        },
        LetterPropState::CurrentGuess(letter) => html! {
            <div class="letter guess">
                {letter}
            </div>
        },
    }
}

/// An enum to represent the state of a [`RowComp`].
///
/// A row can either have a previously guessed word, which be 5 [`Letter`]s, or it can be an
/// in-progress guess, which will be up to 5 characters, or it can be completely empty.
#[derive(Clone, PartialEq)]
enum RowPropState {
    /// This row contains a previously guessed word.
    Concrete(Word),

    /// This row contains an in-progress guess.
    ///
    /// There should only be one row in the board that has this state.
    CurrentGuess(Vec<char>),

    /// This row is empty.
    Empty,
}

/// The props for [`RowComp`].
#[derive(Clone, PartialEq, Properties)]
struct RowProps {
    /// The state of the row.
    state: RowPropState,

    /// Whether or not this row should shake.
    should_shake: bool,
}

/// A component for a single row in the board, with 5 letters.
///
/// See [`RowPropState`] for possible states.
#[function_component(RowComp)]
fn row_comp(props: &RowProps) -> Html {
    let get_letter = |index: usize| -> LetterPropState {
        match &props.state {
            RowPropState::Concrete(word) => LetterPropState::Concrete(word[index]),
            RowPropState::CurrentGuess(guess) => match guess.get(index) {
                None => LetterPropState::Empty,
                Some(c) => LetterPropState::CurrentGuess(*c),
            },
            RowPropState::Empty => LetterPropState::Empty,
        }
    };

    let contents = html! {
        <>
            <LetterComp letter={get_letter(0)} delay=0 />
            <LetterComp letter={get_letter(1)} delay=250 />
            <LetterComp letter={get_letter(2)} delay=500 />
            <LetterComp letter={get_letter(3)} delay=750 />
            <LetterComp letter={get_letter(4)} delay=1000 />
        </>
    };

    let correct_guess = match props.state {
        RowPropState::Concrete(word) => {
            word.iter().map(|l| l.position).collect::<Vec<_>>() == vec![Position::Correct; 5]
        }
        _ => false,
    };

    if props.should_shake {
        // This is a JS Promise that waits for 600ms and then removes the ID of the shaking row
        let _ = Promise::new(&mut |_: Function, _: Function| {
            let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
                    &Function::new_no_args(
                        "let x = document.getElementsByClassName('row-shake'); if (x[0] !== undefined) {x[0].classList.remove('row-shake');}"
                    ),
                    600,
                );
        });

        html! {
            <div class={classes!("row", "row-shake")}>
                {contents}
            </div>
        }
    } else if correct_guess {
        let _ = Promise::new(&mut |_: Function, _: Function| {
            let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
                &Function::new_no_args(
                    "document.getElementById('correct-row').classList.add('row-correct-bounce');",
                ),
                1800,
            );
        });

        html! {
            <div class="row" id="correct-row">
                {contents}
            </div>
        }
    } else {
        html! {
            <div class="row">
                {contents}
            </div>
        }
    }
}

/// The props for [`BoardComp`].
#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    /// A list of previous guesses.
    pub guesses: Vec<Word>,

    /// The guess which is currently being typed.
    ///
    /// This guess is managed by the [`Model`](super::Model) component, which acts as a bridge
    /// between this board and the [`KeyboardComp`](super::keyboard::KeyboardComp).
    pub current_guess: Option<Vec<char>>,

    /// Whether the user has hit enter on a bad guess.
    ///
    /// This prop is used to make the row shake.
    pub bad_guess: bool,
}

/// A component to represent the whole board with all 6 rows.
///
/// The HTML is just 6 [`RowComp`]s wrapped in a div.
#[function_component(BoardComp)]
pub fn board_comp(props: &BoardProps) -> Html {
    let get_row = |index: usize| -> Html {
        if let Some(letters) = props.guesses.get(index) {
            html! {
                <RowComp state={RowPropState::Concrete(*letters)} should_shake={false} />
            }
        } else if index == props.guesses.len() {
            let should_shake = props.bad_guess;
            let state =
                RowPropState::CurrentGuess(props.current_guess.clone().unwrap_or_else(Vec::new));

            html! {
                <RowComp {state} {should_shake} />
            }
        } else {
            html! {
                <RowComp state={RowPropState::Empty} should_shake={false} />
            }
        }
    };

    let style = if let Some((width, height)) = get_window_size() {
        let height = min(height - 260, 420);
        let width = min(width, 5 * height / 6);
        let height = min(height, 6 * width / 5);
        format!("width: {width}px; height: {height}px;")
    } else {
        String::new()
    };

    html! {
        <div {style} class="board">
            {get_row(0)}
            {get_row(1)}
            {get_row(2)}
            {get_row(3)}
            {get_row(4)}
            {get_row(5)}
        </div>
    }
}
