use std::fmt;

#[derive(Default, Debug)]
pub struct Word {
    letters: [String; 5]
}

impl Word {
    pub fn new(word: impl Into<String>) -> Result<Self, &'static str> {
        word.into().try_into()
    }
}

impl TryFrom<String> for Word {
    type Error =  &'static str;

    fn try_from(word: String) -> Result<Self, Self::Error> {
        if word.chars().count() != 5 {
            return Err("String must be 5 chars");
        }
        if !word.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("String must be alphabetic");
        }
        Ok(Self{letters: std::array::from_fn(|i| {
            word.chars().nth(i).unwrap().to_string().to_uppercase()
        })})
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.letters.concat())
    }
}
