mod game;

fn main() {
    let game = game::Game::new("hello".to_string());

    if let Ok(g) = game {
        println!("{:?}", g);
        if g.guess("hello") {
            println!("correct guess");
        }
    }

    let word = game::Word::new("hello");
    let word = game::Word::new("h2llo");
    let word = game::Word::new("helLo");

}
