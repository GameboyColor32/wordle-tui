

#[derive(Default, Debug)]
enum GameState {
    #[default] SplashScreen,
    Playing,
    End
}

#[derive(Default, Debug)]
pub struct Game {
    word: String,
    guesses: [String; 6],
    state: GameState,
    current_input: String,
}

impl Game {
    pub fn new(word: String) -> Result<Self, &'static str> {
        if word.chars().count() > 5 {
            return Err("String must be less than 5 chars");
        }

        Ok(Self {
            word, ..Default::default()
        })
    }
}
