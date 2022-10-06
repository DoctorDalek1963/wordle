use super::{Model, ModelMsg};
use std::collections::HashMap;
use web_sys::MouseEvent;
use wordle::letters::Position;
use yew::{classes, html, html::Scope, Component, Context, Html, Properties};

fn get_parent<PARENT: Component, COMP: Component>(ctx: &Context<COMP>) -> Scope<PARENT> {
    ctx.link().get_parent().unwrap().clone().downcast()
}

struct KeyComp {}

#[derive(Clone, PartialEq, Properties)]
struct KeyProps {
    letter: char,
    position: Option<Position>,
}

impl Component for KeyComp {
    type Message = ();
    type Properties = KeyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        fn position_to_class(position: Option<Position>) -> &'static str {
            match position {
                None => "",
                Some(position) => match position {
                    Position::NotInWord => "notinword",
                    Position::WrongPosition => "wrongposition",
                    Position::Correct => "correct",
                },
            }
        }

        let parent: Scope<KeyboardComp> = get_parent(ctx);
        let letter = ctx.props().letter;
        let position = ctx.props().position;

        let onclick = parent.callback(move |event: MouseEvent| {
            if event.detail() == 0 {
                KeyboardMsg::DoNothing
            } else {
                KeyboardMsg::AddToCurrentGuess(letter)
            }
        });

        html! {
            <button class={classes!("keyboard-key", position_to_class(position))} {onclick}>{ ctx.props().letter }</button>
        }
    }
}

struct EnterKeyComp {}

impl Component for EnterKeyComp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let parent: Scope<KeyboardComp> = get_parent(ctx);
        let onclick = parent.callback(move |_| KeyboardMsg::SendEnter);
        html! {
            <button class="keyboard-key special-key" {onclick}>{ "ENTER" }</button>
        }
    }
}

struct BackspaceKeyComp {}

impl Component for BackspaceKeyComp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let parent: Scope<KeyboardComp> = get_parent(ctx);
        let onclick = parent.callback(move |_| KeyboardMsg::SendBackspace);
        html! {
            <button class="keyboard-key special-key" {onclick}>
                <svg viewBox="0 0 24 24" height="24" width="24">
                    <path fill="var(--color-tone-1)" d="M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.9.89 1.59.89h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H7.07L2.4 12l4.66-7H22v14zm-11.59-2L14 13.41 17.59 17 19 15.59 15.41 12 19 8.41 17.59 7 14 10.59 10.41 7 9 8.41 12.59 12 9 15.59z" />
                </svg>
            </button>
        }
    }
}

pub struct KeyboardComp {}

#[derive(Clone, PartialEq, Properties)]
pub struct KeyboardProps {
    pub map: HashMap<char, Option<Position>>,
}

pub enum KeyboardMsg {
    DoNothing,
    AddToCurrentGuess(char),
    SendEnter,
    SendBackspace,
}

impl Component for KeyboardComp {
    type Message = KeyboardMsg;
    type Properties = KeyboardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let parent: Scope<Model> = get_parent(ctx);
        parent
            .callback(move |_| match msg {
                Self::Message::DoNothing => ModelMsg::DoNothing,
                Self::Message::AddToCurrentGuess(letter) => ModelMsg::AddToCurrentGuess(letter),
                Self::Message::SendEnter => ModelMsg::SendEnter,
                Self::Message::SendBackspace => ModelMsg::SendBackspace,
            })
            .emit(());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let get_key = |letter: char| -> Html {
            let position = *ctx.props().map.get(&letter).unwrap_or_else(|| {
                panic!("We should have a position value for character {:?}", letter)
            });

            html! {
                <KeyComp {letter} {position} />
            }
        };

        html! {
            <div class="keyboard">
                <div class="keyboard-row">
                    {get_key('Q')}
                    {get_key('W')}
                    {get_key('E')}
                    {get_key('R')}
                    {get_key('T')}
                    {get_key('Y')}
                    {get_key('U')}
                    {get_key('I')}
                    {get_key('O')}
                    {get_key('P')}
                </div>
                <div class="keyboard-row">
                    <div class="keyboard-spacer" />
                    {get_key('A')}
                    {get_key('S')}
                    {get_key('D')}
                    {get_key('F')}
                    {get_key('G')}
                    {get_key('H')}
                    {get_key('J')}
                    {get_key('K')}
                    {get_key('L')}
                    <div class="keyboard-spacer" />
                </div>
                <div class="keyboard-row">
                    <EnterKeyComp />
                    {get_key('Z')}
                    {get_key('X')}
                    {get_key('C')}
                    {get_key('V')}
                    {get_key('B')}
                    {get_key('N')}
                    {get_key('M')}
                    <BackspaceKeyComp />
                </div>
            </div>
        }
    }
}
