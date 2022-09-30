use board::BoardComp;
use rand::seq::SliceRandom;
use wordle::{letters::Letter, Game};
use yew::prelude::*;

mod board;

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
