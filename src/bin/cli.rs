use wordle;

fn main() {
    let game = wordle::Game::new();
    println!("{:?}", game.make_guess("dyson").unwrap());
}
