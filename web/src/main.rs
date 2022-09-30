use board::BoardComp;
use wordle::Game;
use yew::prelude::*;

mod board;

struct Model {
    game: Game,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { game: Game::new() }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <BoardComp game={self.game.clone()}/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
