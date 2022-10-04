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

pub struct KeyboardComp {}

#[derive(Clone, PartialEq, Properties)]
pub struct KeyboardProps {}

pub enum KeyboardMsg {
    AddToCurrentGuess(char),
}

impl Component for KeyboardComp {
    type Message = KeyboardMsg;
    type Properties = KeyboardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::AddToCurrentGuess(letter) => {
                let parent: Scope<Model> = get_parent(ctx);
                parent
                    .callback(move |_| ModelMsg::AddToCurrentGuess(letter))
                    .emit(());
                true
            }
        }
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
                    <KeyComp letter={'a'} />
                    <KeyComp letter={'s'} />
                    <KeyComp letter={'d'} />
                    <KeyComp letter={'f'} />
                    <KeyComp letter={'g'} />
                    <KeyComp letter={'h'} />
                    <KeyComp letter={'j'} />
                    <KeyComp letter={'k'} />
                    <KeyComp letter={'l'} />
                </div>
                <div class="keyboard-row">
                    <KeyComp letter={'z'} />
                    <KeyComp letter={'x'} />
                    <KeyComp letter={'c'} />
                    <KeyComp letter={'v'} />
                    <KeyComp letter={'b'} />
                    <KeyComp letter={'n'} />
                    <KeyComp letter={'m'} />
                </div>
            </div>
        }
    }
}
