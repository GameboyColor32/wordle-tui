mod game;

fn main() {
    let game = game::Game::new("hello".to_string());
    
    if let Ok(g) = game {
        println!("{:?}", g);
    }
}
