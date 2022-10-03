use yew::{html, html::Scope, Component, Context, Html, Properties};

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
        html! {
            <button class="keyboard-key">{ ctx.props().letter }</button>
        }
    }
}

pub struct KeyboardComp {}

#[derive(Clone, PartialEq, Properties)]
pub struct KeyboardProps {}

impl Component for KeyboardComp {
    type Message = ();
    type Properties = KeyboardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use super::{Model, ModelMsg};

        let parent: Scope<Model> = ctx.link().get_parent().unwrap().clone().downcast();

        html! {
            <div class="keyboard">
                <button onclick={parent.callback(|_| ModelMsg::MakeGuess("booby".to_string()))}>{ "Keyboard Button" }</button>
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
                <div>
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
