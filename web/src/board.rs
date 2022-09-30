use wordle::{
    letters::{Letter, Position},
    Game,
};
use yew::{classes, html, Component, Context, Html, Properties};

struct LetterComp {
    props: LetterProps,
}

#[derive(Clone, PartialEq, Properties)]
struct LetterProps {
    delay: u32,
    letter: Option<Letter>,
}

impl Component for LetterComp {
    type Message = ();
    type Properties = LetterProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        fn position_to_class(letter: Letter) -> &'static str {
            match letter.position {
                Position::NotInWord => "letter-notinword",
                Position::WrongPosition => "letter-wrongposition",
                Position::Correct => "letter-correct",
            }
        }

        match self.props.letter {
            None => html! {
                <div class="letter" style={format!("animation-delay: {}ms;", self.props.delay)} />
            },
            Some(letter) => html! {
                <div class={classes!("letter", position_to_class(letter))} style={format!("animation-delay: {}ms;", self.props.delay)}>
                    {letter.letter}
                </div>
            },
        }
    }
}

pub struct RowComp {
    props: RowProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RowProps {
    letters: Option<[Letter; 5]>,
}

pub struct NewGuessMessage([Letter; 5]);

impl Component for RowComp {
    type Message = NewGuessMessage;
    type Properties = RowProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.props.letters {
            None => html! {
                <div class="row">
                    <LetterComp letter={None} delay=0 />
                    <LetterComp letter={None} delay=100 />
                    <LetterComp letter={None} delay=200 />
                    <LetterComp letter={None} delay=300 />
                    <LetterComp letter={None} delay=400 />
                </div>
            },
            Some(letters) => html! {
                <div class="row">
                    <LetterComp letter={letters[0]} delay=0 />
                    <LetterComp letter={letters[1]} delay=100 />
                    <LetterComp letter={letters[2]} delay=200 />
                    <LetterComp letter={letters[3]} delay=300 />
                    <LetterComp letter={letters[4]} delay=400 />
                </div>
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.props.letters = Some(msg.0);
        true
    }
}

pub struct BoardComp {
    props: BoardProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    pub game: Game,
}

impl Component for BoardComp {
    type Message = NewGuessMessage;
    type Properties = BoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="board">
                <RowComp letters={None} />
                <RowComp letters={None} />
                <RowComp letters={None} />
                <RowComp letters={None} />
                <RowComp letters={None} />
                <RowComp letters={None} />
            </div>
        }
    }
}
