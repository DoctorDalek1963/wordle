use board::BoardComp;
use rand::seq::SliceRandom;
use wordle::{letters::Letter, Game};
use yew::prelude::*;

mod board;

fn storage_get_dark_mode() -> Option<bool> {
    let storage = web_sys::window()?.local_storage().unwrap_or(None)?;
    match storage.get_item("darkMode") {
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

fn storage_set_dark_mode(dark_mode: bool) -> Option<()> {
    let storage = web_sys::window()?.local_storage().unwrap_or(None)?;
    match storage.set_item("darkMode", &dark_mode.to_string()) {
        Err(_) => None,
        Ok(_) => Some(()),
    }
}

fn set_dark_mode(dark_mode: bool) -> Option<()> {
    let class_list = web_sys::window()?.document()?.body()?.class_list();

    let a: &str;
    let b: &str;

    match dark_mode {
        true => {
            a = "light";
            b = "dark";
        }
        false => {
            a = "dark";
            b = "light";
        }
    };

    if class_list.contains(a) {
        if let Err(_) = class_list.remove_1(a) {
            return None;
        };
    };

    if let Err(_) = class_list.add_1(b) {
        return None;
    };

    Some(())
}

struct Model {
    game: Game,
    guesses: Vec<[Letter; 5]>,
}

struct MakeGuess(String);

impl Component for Model {
    type Message = MakeGuess;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: Game::new(),
            guesses: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let guess = msg.0;
        match self.game.make_guess(&guess) {
            Ok(letters) => {
                self.guesses.push(letters);
            }
            Err(_) => unimplemented!(),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = storage_get_dark_mode().unwrap_or(false);
        set_dark_mode(dark_mode);

        let link = ctx.link();
        html! {
            <div class="game">
                <div class="board-container">
                    <BoardComp game={self.game.clone()} guesses={self.guesses.clone()} />
                    if self.guesses.len() < 6 {
                        <button onclick={link.callback(|_| MakeGuess(
                                wordle::valid_words::VALID_WORDS
                                .choose(&mut rand::thread_rng())
                                .unwrap()
                                .to_string()
                        ))}>{ "BUTTON" }</button>
                    } else {}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
