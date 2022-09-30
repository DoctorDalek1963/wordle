use wordle::{
    letters::{Letter, Position},
    Game,
};
use yew::{classes, html, Component, Context, Html, Properties};

struct LetterComp {}

#[derive(Clone, PartialEq, Properties)]
struct LetterProps {
    delay: u32,
    letter: Option<Letter>,
}

impl Component for LetterComp {
    type Message = ();
    type Properties = LetterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        fn position_to_class(letter: Letter) -> &'static str {
            match letter.position {
                Position::NotInWord => "notinword",
                Position::WrongPosition => "wrongposition",
                Position::Correct => "correct",
            }
        }

        match ctx.props().letter {
            None => html! {
                <div class="letter" style={format!("animation-delay: {}ms;", ctx.props().delay)} />
            },
            Some(letter) => html! {
                <div class={classes!("letter", position_to_class(letter))} style={format!("animation-delay: {}ms;", ctx.props().delay)}>
                    {letter.letter}
                </div>
            },
        }
    }
}

pub struct RowComp {}

#[derive(Clone, PartialEq, Properties)]
pub struct RowProps {
    letters: Option<[Letter; 5]>,
}

impl Component for RowComp {
    type Message = ();
    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        fn get_option_letter(letters: Option<[Letter; 5]>, index: usize) -> Option<Letter> {
            Some(letters?[index])
        }

        html! {
            <div class="row">
                <LetterComp letter={get_option_letter(ctx.props().letters, 0)} delay=0 />
                <LetterComp letter={get_option_letter(ctx.props().letters, 1)} delay=100 />
                <LetterComp letter={get_option_letter(ctx.props().letters, 2)} delay=200 />
                <LetterComp letter={get_option_letter(ctx.props().letters, 3)} delay=300 />
                <LetterComp letter={get_option_letter(ctx.props().letters, 4)} delay=400 />
            </div>
        }
    }
}

pub struct BoardComp {}

#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    pub game: Game,
    pub guesses: Vec<[Letter; 5]>,
}

impl Component for BoardComp {
    type Message = ();
    type Properties = BoardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        fn deref_option_value<T: Clone>(x: Option<&T>) -> Option<T> {
            Some(x?.clone())
        }

        html! {
            <div class="board">
                <RowComp letters={deref_option_value(ctx.props().guesses.get(0))} />
                <RowComp letters={deref_option_value(ctx.props().guesses.get(1))} />
                <RowComp letters={deref_option_value(ctx.props().guesses.get(2))} />
                <RowComp letters={deref_option_value(ctx.props().guesses.get(3))} />
                <RowComp letters={deref_option_value(ctx.props().guesses.get(4))} />
                <RowComp letters={deref_option_value(ctx.props().guesses.get(5))} />
            </div>
        }
    }
}
