use std::{
    collections::HashSet,
    sync::LazyLock,
};

const WORDS: &str = include_str!("../data/words.txt");

static WORDLE: LazyLock<Lexicon> = LazyLock::new(|| Lexicon::from_embedded(WORDS));

pub struct Lexicon {
    words: Vec<&'static str>,
    lookup: HashSet<&'static str>,
}

impl Lexicon {
    fn from_embedded(source: &'static str) -> Self {
        let words: Vec<_> = source
            .lines()
            .map(str::trim)
            .filter(|word| !word.is_empty())
            .collect();
        let lookup = words.iter().copied().collect();

        Self { words, lookup }
    }

    pub fn contains(&self, word: &str) -> bool {
        let normalized = word.to_ascii_lowercase();

        self.lookup.contains(normalized.as_str())
    }

    pub fn get(&self, index: usize) -> Option<&'static str> {
        self.words.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }
}

pub fn wordle() -> &'static Lexicon {
    &WORDLE
}
