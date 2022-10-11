//! This crate is a simple web interface to [`wordle`](::wordle) using
//! [`yew`](https://docs.rs/yew/0.19.3/yew/).

use crate::{board::BoardComp, keyboard::KeyboardComp};
use gloo_events::EventListener;
use gloo_timers::callback::Timeout;
use gloo_utils::{body, document, window};
use std::cell::RefCell;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{KeyboardEvent, MouseEvent};
use wordle::{letters::Letter, valid_words::ALPHABET, Game};
use yew::{html, Component, Context, Html};

mod board;
mod keyboard;

/// Get the value of the `wordleDarkMode` key in `localStorage`.
fn storage_get_dark_mode() -> Option<bool> {
    let storage = window().local_storage().unwrap_or(None)?;
    match storage.get_item("wordleDarkMode") {
        Err(_) => None,
        Ok(opt_str) => match opt_str {
            None => None,
            Some(value) => {
                if value == "true" {
                    Some(true)
                } else if value == "false" {
                    Some(false)
                } else {
                    None
                }
            }
        },
    }
}

/// Set the value of the `wordleDarkMode` key in `localStorage`.
fn storage_set_dark_mode(dark_mode: bool) -> Option<()> {
    let storage = window().local_storage().unwrap_or(None)?;
    match storage.set_item("wordleDarkMode", &dark_mode.to_string()) {
        Err(_) => None,
        Ok(_) => Some(()),
    }
}

/// Set dark mode on the body of the HTML by adding or removing the "dark" class.
fn set_dark_mode(dark_mode: bool) -> Option<()> {
    let class_list = body().class_list();

    let a: &str;
    let b: &str;

    if dark_mode {
        a = "light";
        b = "dark";
    } else {
        a = "dark";
        b = "light";
    };

    if class_list.contains(a) {
        if class_list.remove_1(a).is_err() {
            return None;
        };
    };

    if class_list.add_1(b).is_err() {
        return None;
    };

    Some(())
}

/// The root component of the app.
struct Model {
    /// The Wordle game itself.
    game: Game,

    /// A list of previously guessed words.
    guesses: Vec<[Letter; 5]>,

    /// The guess which is currently being typed.
    current_guess: Option<Vec<char>>,

    /// The event listener for keyboard events.
    ///
    /// We need to keep this in the struct to avoid it being dropped from the DOM and being
    /// useless. It's an [`Option`] because we can't initialise it until we have the
    /// [`Context`](https://docs.rs/yew/0.19.3/yew/prelude/struct.Context.html), which we do in
    /// [`Model::rendered`].
    kbd_listener: Option<EventListener>,

    /// Whether the user has just submitted a bad guess - meaning the guess row should shake.
    ///
    /// The bool is wrapped in a [`RefCell`] to allow it to be mutated in [`view()`](Model::view).
    bad_guess: RefCell<bool>,
}

/// An enum of messages that can be sent to the model.
#[derive(Clone)]
pub enum ModelMsg {
    /// Do nothing.
    ///
    /// This message is needed because the keyboard listener triggers on every keypress, but we
    /// want to ignore some of them. We also want to ignore when a key button on the
    /// [`KeyboardComp`] is triggered by hitting enter when it's selected, rather than by a mouse click.
    DoNothing,

    /// Force update.
    ///
    /// This should only be used internally.
    ForceUpdate,

    /// Make a guess with the given string. This will call [`Game::make_guess`].
    MakeGuess(String),

    /// Toggle dark mode for the whole HTML body.
    ///
    /// See [`set_dark_mode`].
    ToggleDarkMode,

    /// The given character to the current guess.
    AddToCurrentGuess(char),

    /// This message represents the enter key being pressed, meaning the user wants to submit their
    /// current guess.
    SendEnter,

    /// This message represents the backspace key being pressed, meaning the user wants to delete
    /// the last character they added to their guess.
    SendBackspace,
}

impl Component for Model {
    type Message = ModelMsg;

    /// This component has no props.
    type Properties = ();

    /// Create a simple, default struct for the component.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: Game::new(),
            guesses: Vec::new(),
            current_guess: None,
            kbd_listener: None,
            bad_guess: RefCell::new(false),
        }
    }

    /// Update the model based on the given message. See [`ModelMsg`].
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use wordle::GuessError;

        match msg {
            Self::Message::DoNothing => false,
            Self::Message::ForceUpdate => true,
            Self::Message::MakeGuess(guess) => {
                match self.game.make_guess(&guess) {
                    Ok(letters) => {
                        self.guesses.push(letters);
                        self.current_guess = None;
                    }
                    Err(e) => match e {
                        GuessError::WrongWordLength => unreachable!("The player should only be able to submit a guess with 5 letters, not {}", guess.len()),
                        GuessError::IncludesNonAscii => unreachable!("The guess should never be able to contain non-ASCII characters (guess = {guess:?})"),
                        GuessError::InvalidWord => {
                            self.bad_guess.replace(true);
                        }
                    }
                };
                true
            }
            Self::Message::ToggleDarkMode => {
                let dark_mode = storage_get_dark_mode().unwrap_or(false);
                storage_set_dark_mode(!dark_mode);
                true
            }
            Self::Message::AddToCurrentGuess(letter) => {
                match self.current_guess.as_mut() {
                    Some(letters) => {
                        if letters.len() < 5 {
                            letters.push(letter);
                        }
                    }
                    None => self.current_guess = Some(vec![letter]),
                };
                true
            }
            Self::Message::SendEnter => {
                if let Some(chars) = &self.current_guess {
                    if chars.len() == 5 {
                        let guess: String = chars.iter().collect();
                        self.update(ctx, Self::Message::MakeGuess(guess.to_uppercase()))
                    } else {
                        self.bad_guess.replace(true);
                        true
                    }
                } else {
                    self.bad_guess.replace(true);
                    true
                }
            }
            Self::Message::SendBackspace => {
                if let Some(chars) = &mut self.current_guess {
                    if chars.len() > 0 {
                        chars.pop();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Return the HTML of the whole model.
    ///
    /// This includes the header with dark mode button, the game board, and the virtual keyboard.
    /// It also sets up a keyboard listener to allow the user to type.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = storage_get_dark_mode().unwrap_or(false);
        set_dark_mode(dark_mode);

        let button_icon: Html = if dark_mode {
            html! {
                <svg viewBox="0 0 24 24" width="24" height="24">
                    <path fill="currentColor" d="M9.37,5.51C9.19,6.15,9.1,6.82,9.1,7.5c0,4.08,3.32,7.4,7.4,7.4c0.68,0,1.35-0.09,1.99-0.27C17.45,17.19,14.93,19,12,19 c-3.86,0-7-3.14-7-7C5,9.07,6.81,6.55,9.37,5.51z M12,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9c0-0.46-0.04-0.92-0.1-1.36 c-0.98,1.37-2.58,2.26-4.4,2.26c-2.98,0-5.4-2.42-5.4-5.4c0-1.81,0.89-3.42,2.26-4.4C12.92,3.04,12.46,3,12,3L12,3z" />
                </svg>
            }
        } else {
            html! {
                <svg viewBox="0 0 24 24" width="24" height="24">
                    <path fill="currentColor" d="M12,9c1.65,0,3,1.35,3,3s-1.35,3-3,3s-3-1.35-3-3S10.35,9,12,9 M12,7c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5 S14.76,7,12,7L12,7z M2,13l2,0c0.55,0,1-0.45,1-1s-0.45-1-1-1l-2,0c-0.55,0-1,0.45-1,1S1.45,13,2,13z M20,13l2,0c0.55,0,1-0.45,1-1 s-0.45-1-1-1l-2,0c-0.55,0-1,0.45-1,1S19.45,13,20,13z M11,2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V2c0-0.55-0.45-1-1-1S11,1.45,11,2z M11,20v2c0,0.55,0.45,1,1,1s1-0.45,1-1v-2c0-0.55-0.45-1-1-1C11.45,19,11,19.45,11,20z M5.99,4.58c-0.39-0.39-1.03-0.39-1.41,0 c-0.39,0.39-0.39,1.03,0,1.41l1.06,1.06c0.39,0.39,1.03,0.39,1.41,0s0.39-1.03,0-1.41L5.99,4.58z M18.36,16.95 c-0.39-0.39-1.03-0.39-1.41,0c-0.39,0.39-0.39,1.03,0,1.41l1.06,1.06c0.39,0.39,1.03,0.39,1.41,0c0.39-0.39,0.39-1.03,0-1.41 L18.36,16.95z M19.42,5.99c0.39-0.39,0.39-1.03,0-1.41c-0.39-0.39-1.03-0.39-1.41,0l-1.06,1.06c-0.39,0.39-0.39,1.03,0,1.41 s1.03,0.39,1.41,0L19.42,5.99z M7.05,18.36c0.39-0.39,0.39-1.03,0-1.41c-0.39-0.39-1.03-0.39-1.41,0l-1.06,1.06 c-0.39,0.39-0.39,1.03,0,1.41s1.03,0.39,1.41,0L7.05,18.36z" />
                </svg>
            }
        };

        let onclick = ctx.link().callback(|event: MouseEvent| {
            if event.detail() == 0 {
                ModelMsg::DoNothing
            } else {
                ModelMsg::ToggleDarkMode
            }
        });

        let bad_guess = self.bad_guess.replace(false);

        if bad_guess {
            let link = ctx.link().clone();
            Timeout::new(600, move || link.send_message(ModelMsg::ForceUpdate)).forget();
        };

        html! {
            <>
            <header>
                <div class="wordle-title">
                    <div class="main-title">{ "Wordle" }</div>
                    <div class="subtitle">{ "by Dyson" }</div>
                </div>
                <div>
                    <button class="dark-mode-button" {onclick}>
                        {button_icon}
                    </button>
                </div>
            </header>
            <div class="game">
                <div class="board-container">
                    <BoardComp guesses={self.guesses.clone()} current_guess={self.current_guess.clone()} {bad_guess} />
                </div>
                <KeyboardComp map={self.game.keyboard.clone()} />
            </div>
            </>
        }
    }

    /// This function is run after the HTML is generated, but just before the component is run.
    ///
    /// If this is the first time we're rendering the model, then we set up an
    /// [`EventListener`](https://docs.rs/web-sys/0.3.60/web_sys/struct.EventListener.html)
    /// on the document to listen for
    /// [`KeyboardEvent`](https://docs.rs/web-sys/0.3.60/web_sys/struct.KeyboardEvent.html)s
    /// and update the model accordingly when the user types on their keyboard.
    ///
    /// See [`Model::kbd_listener`].
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let callback = ctx.link().callback(|event: KeyboardEvent| {
            let key = event.key().to_ascii_lowercase();
            if key.len() == 1
                && ALPHABET.contains(&key.to_ascii_uppercase().chars().next().unwrap())
            {
                Self::Message::AddToCurrentGuess(key.chars().next().unwrap())
            } else if key == "enter" {
                Self::Message::SendEnter
            } else if key == "backspace" {
                Self::Message::SendBackspace
            } else {
                Self::Message::DoNothing
            }
        });

        let document = document();

        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw();
            callback.emit(event.clone());
        });

        self.kbd_listener.replace(listener);
    }
}

/// Run the [`yew`](https://docs.rs/yew/0.19.3/yew/) app.
fn main() {
    yew::start_app::<Model>();
}
