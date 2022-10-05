use super::{Model, ModelMsg};
use yew::{html, html::Scope, Component, Context, Html, Properties};

fn get_parent<PARENT: Component, COMP: Component>(ctx: &Context<COMP>) -> Scope<PARENT> {
    ctx.link().get_parent().unwrap().clone().downcast()
}

struct KeyComp {}

#[derive(Clone, PartialEq, Properties)]
struct KeyProps {
    letter: char,
}

impl Component for KeyComp {
    type Message = ();
    type Properties = KeyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let parent: Scope<KeyboardComp> = get_parent(ctx);
        let letter = ctx.props().letter;
        let onclick = parent.callback(move |_| KeyboardMsg::AddToCurrentGuess(letter));

        html! {
            <button class="keyboard-key" {onclick}>{ ctx.props().letter }</button>
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
pub struct KeyboardProps {}

pub enum KeyboardMsg {
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
                Self::Message::AddToCurrentGuess(letter) => ModelMsg::AddToCurrentGuess(letter),
                Self::Message::SendEnter => ModelMsg::SendEnter,
                Self::Message::SendBackspace => ModelMsg::SendBackspace,
            })
            .emit(());
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="keyboard">
                <div class="keyboard-row">
                    <KeyComp letter={'q'} />
                    <KeyComp letter={'w'} />
                    <KeyComp letter={'e'} />
                    <KeyComp letter={'r'} />
                    <KeyComp letter={'t'} />
                    <KeyComp letter={'y'} />
                    <KeyComp letter={'u'} />
                    <KeyComp letter={'i'} />
                    <KeyComp letter={'o'} />
                    <KeyComp letter={'p'} />
                </div>
                <div class="keyboard-row">
                    <div class="keyboard-spacer" />
                    <KeyComp letter={'a'} />
                    <KeyComp letter={'s'} />
                    <KeyComp letter={'d'} />
                    <KeyComp letter={'f'} />
                    <KeyComp letter={'g'} />
                    <KeyComp letter={'h'} />
                    <KeyComp letter={'j'} />
                    <KeyComp letter={'k'} />
                    <KeyComp letter={'l'} />
                    <div class="keyboard-spacer" />
                </div>
                <div class="keyboard-row">
                    <EnterKeyComp />
                    <KeyComp letter={'z'} />
                    <KeyComp letter={'x'} />
                    <KeyComp letter={'c'} />
                    <KeyComp letter={'v'} />
                    <KeyComp letter={'b'} />
                    <KeyComp letter={'n'} />
                    <KeyComp letter={'m'} />
                    <BackspaceKeyComp />
                </div>
            </div>
        }
    }
}
