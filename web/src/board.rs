use wordle::{
    letters::{Letter, Position},
    Game,
};
use yew::{classes, html, Component, Context, Html, Properties};

struct LetterComp {}

#[derive(Clone, PartialEq)]
enum LetterPropConcreteOrGuess {
    Concrete(Letter),
    Guess(char),
    Empty,
}

#[derive(Clone, PartialEq, Properties)]
struct LetterProps {
    delay: u32,
    letter: LetterPropConcreteOrGuess,
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
            LetterPropConcreteOrGuess::Empty => html! {
                <div class="letter empty" />
            },
            LetterPropConcreteOrGuess::Concrete(letter) => html! {
                <div class={classes!("letter", position_to_class(letter))} style={format!("animation-delay: {}ms;", ctx.props().delay)}>
                    {letter.letter}
                </div>
            },
            LetterPropConcreteOrGuess::Guess(letter) => html! {
                <div class="letter guess">
                    {letter}
                </div>
            },
        }
    }
}

struct RowComp {}

#[derive(Clone, PartialEq, Properties)]
struct RowProps {
    letters: Option<[Letter; 5]>,
    current_guess: Option<Vec<char>>,
}

impl Component for RowComp {
    type Message = ();
    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_letter = |index: usize| -> LetterPropConcreteOrGuess {
            let props = ctx.props();
            if props.letters.is_some() && props.current_guess.is_some() {
                unreachable!(
                    concat!(
                        "We should never have a row with a fixed guess and a current guess\n",
                        "  letters = {:?}\n",
                        "  current_guess = {:?}"
                    ),
                    props.letters, props.current_guess
                );
            } else if let Some(letters) = props.letters {
                LetterPropConcreteOrGuess::Concrete(letters[index])
            } else if let Some(guess) = &props.current_guess {
                match guess.get(index) {
                    Some(c) => LetterPropConcreteOrGuess::Guess(*c),
                    None => LetterPropConcreteOrGuess::Empty,
                }
            } else {
                LetterPropConcreteOrGuess::Empty
            }
        };

        html! {
            <div class="row">
                <LetterComp letter={get_letter(0)} delay=0 />
                <LetterComp letter={get_letter(1)} delay=100 />
                <LetterComp letter={get_letter(2)} delay=200 />
                <LetterComp letter={get_letter(3)} delay=300 />
                <LetterComp letter={get_letter(4)} delay=400 />
            </div>
        }
    }
}

pub struct BoardComp {}

#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    pub game: Game,
    pub guesses: Vec<[Letter; 5]>,
    pub current_guess: Option<Vec<char>>,
}

impl Component for BoardComp {
    type Message = ();
    type Properties = BoardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_row = |index: usize| -> Html {
            let props = ctx.props();

            if let Some(letters) = props.guesses.get(index) {
                html! {
                    <RowComp letters={letters.clone()} current_guess={None} />
                }
            } else {
                if index == props.guesses.len() {
                    html! {
                        <RowComp letters={None} current_guess={props.current_guess.clone()} />
                    }
                } else {
                    html! {
                        <RowComp letters={None} current_guess={None} />
                    }
                }
            }
        };

        html! {
            <div class="board">
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
