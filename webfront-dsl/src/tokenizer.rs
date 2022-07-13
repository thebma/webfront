use std::str;
use std::cell::RefCell;

use crate::lexicon::Lexicon;
use crate::token::Token;

pub struct Tokenizer<'slice> {
    source: &'slice[u8],
    lexicon: RefCell<Lexicon>,
    output: Vec<Token>
}

impl<'slice> Tokenizer<'slice> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer { 
            source: source.as_bytes(),
            lexicon: RefCell::from(Lexicon::new(false)),
            output: Vec::new()
        }
    }

    pub fn tokenize(&mut self) {
        for token in self.source.iter() {
            let match_value = self.lexicon.borrow_mut().advance(token.clone());
        }

        //TODO: Handle remaining values, or implement some kind of EOF function inside lexicon.
    } 
}