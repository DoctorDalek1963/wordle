//! This module handles components for the game board itself - the 6 rows of 5 letter words.

use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct ShowCorrectGuessProps {
    pub word: String,
}

#[function_component(ShowCorrectGuess)]
pub fn show_correct_guess(props: &ShowCorrectGuessProps) -> Html {
    html! {
        <div class="correct-guess-popup-container">
            <div class="correct-guess-popup">
                {props.word.clone()}
            </div>
        </div>
    }
}
