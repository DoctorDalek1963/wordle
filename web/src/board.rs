//! This module handles components for the game board itself - the 6 rows of 5 letter words.

use gloo_utils::window;
use js_sys::{Function, Promise};
use wordle::letters::{Letter, Position};
use yew::{classes, html, Component, Context, Html, Properties};

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

/// A component for a single letter in a row.
struct LetterComp {}

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

impl Component for LetterComp {
    /// This component accepts no messages.
    type Message = ();

    type Properties = LetterProps;

    /// Create an empty struct.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    /// Return the HTML for this letter component, based on its props.
    ///
    /// See [`LetterPropState`] for possible states.
    fn view(&self, ctx: &Context<Self>) -> Html {
        fn position_to_class(letter: Letter) -> &'static str {
            match letter.position {
                Position::NotInWord => "notinword",
                Position::WrongPosition => "wrongposition",
                Position::Correct => "correct",
            }
        }

        match ctx.props().letter {
            LetterPropState::Empty => html! {
                <div class="letter empty" />
            },
            LetterPropState::Concrete(letter) => html! {
                <div class={classes!("letter", position_to_class(letter))} style={format!("animation-delay: {}ms;", ctx.props().delay)}>
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
}

/// A component for a single row in the board, with 5 letters.
struct RowComp {}

/// An enum to represent the state of a [`RowComp`].
///
/// A row can either have a previously guessed word, which be 5 [`Letter`]s, or it can be an
/// in-progress guess, which will be up to 5 characters, or it can be completely empty.
#[derive(Clone, PartialEq)]
enum RowPropState {
    /// This row contains a previously guessed word.
    Concrete([Letter; 5]),

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

impl Component for RowComp {
    /// This component accepts no messages.
    type Message = ();

    type Properties = RowProps;

    /// Create an empty struct.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    /// Return the HTML of the row, based on its state.
    ///
    /// See [`RowPropState`] for possible states.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_letter = |index: usize| -> LetterPropState {
            let props = ctx.props();
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

        if ctx.props().should_shake {
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
        } else {
            html! {
                <div class="row">
                    {contents}
                </div>
            }
        }
    }
}

/// A component to represent the whole board with all 6 rows.
pub struct BoardComp {}

/// The props for [`BoardComp`].
#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    /// A list of previous guesses as [`Letter`]s with associated positions
    pub guesses: Vec<[Letter; 5]>,

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

impl Component for BoardComp {
    /// This component accepts no messages.
    type Message = ();

    type Properties = BoardProps;

    /// Create an empty struct.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    /// Return the HTML of the board, which is just 6 [`RowComp`]s wrapped in a div.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_row = |index: usize| -> Html {
            let props = ctx.props();

            if let Some(letters) = props.guesses.get(index) {
                html! {
                    <RowComp state={RowPropState::Concrete(*letters)} should_shake={false} />
                }
            } else if index == props.guesses.len() {
                let should_shake = props.bad_guess;
                let state = RowPropState::CurrentGuess(
                    props.current_guess.clone().unwrap_or_else(Vec::new),
                );

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
}
