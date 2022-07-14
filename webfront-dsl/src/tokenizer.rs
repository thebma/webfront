use std::str;
use std::cell::RefCell;

use crate::lexicon::Lexicon;
use crate::lexicon::LexiconMatch;
use crate::token::Token;

pub struct Tokenizer<'slice> {
    source: &'slice[u8],
    lexicon: RefCell<Lexicon>,
}

impl<'slice> Tokenizer<'slice> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer { 
            source: source.as_bytes(),
            lexicon: RefCell::from(Lexicon::new(false)),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        for token in self.source.iter() {
            let result = self.lexicon.borrow_mut().advance(token.clone());

            if let LexiconMatch::Resolved(token) = &result {
                tokens.push(token.clone());
            } else if let LexiconMatch::Illegal() = &result {
                panic!("Illegal syntax!");
            }
        }

        let result = self.lexicon.borrow_mut().end_of_source();

        if let LexiconMatch::Resolved(token) = &result {
            tokens.push(token.clone());
        } else if let LexiconMatch::Illegal() = &result {
            panic!("Illegal syntax!");
        }

        return tokens;
    } 
}