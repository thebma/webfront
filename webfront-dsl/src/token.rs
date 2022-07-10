#[derive(Debug, Clone)]
pub enum TokenMeaning {
    TestValue,
    OpenScope,
    CloseScope
}

#[derive(Debug, Clone)]
pub struct Token {
    pub word: String,
    pub state: TokenMeaning
}

impl Token {
    pub fn new(word: &str, state: TokenMeaning) -> Self {
        Token {
            word: word.to_owned(),
            state: state
        }
    }

    pub fn equals(&self, tokens: &[u8]) -> bool {
        let word_bytes = self.word.as_bytes();

        if word_bytes.len() != tokens.len() {
            return false;
        }

        for n in 0..word_bytes.len() {
            if tokens[n] != word_bytes[n] {
                return false;
            }
        }

        return true;
    }

    pub fn starts_with(&self, tokens: &[u8]) -> bool {
        let word_bytes = self.word.as_bytes();

        for n in 0..tokens.len() {
            if word_bytes[n] != tokens[n] {
                return false;
            }
        }

        return true;
    }
}