mod game;

use crate::game::Word;

fn main() {
    let game = game::Game::new("hello".to_string());

    if let Ok(g) = game {
        println!("{:?}", g);
        if g.guess("hello") {
            println!("correct guess");
        }
    }

    let word = Word::new("hello");
    println!("{:?}", word);
    let word = Word::new("h2llo");
    println!("{:?}", word);
    let word = Word::new("helLo");
    println!("{:?}", word);
}
