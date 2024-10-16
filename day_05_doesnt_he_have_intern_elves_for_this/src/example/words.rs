use super::word::Word;

pub struct Words {
    words: Vec<String>,
}

impl Words {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    pub fn count_nice_words(&self) -> usize {
        self.words.iter().filter(|word| Word::is_nice(word)).count()
    }

    pub fn count_nicer_words(&self) -> usize {
        self.words
            .iter()
            .filter(|word| Word::is_nicer(word))
            .count()
    }
}
