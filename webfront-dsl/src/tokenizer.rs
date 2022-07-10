use std::str;
use std::cell::RefCell;

use crate::lexicon::Lexicon;
use crate::lexicon::LexiconMatch;

use crate::token::Token;

pub struct Tokenizer<'slice> {
    source: &'slice[u8],
    lexicon: RefCell<Lexicon>,
    source_tokens: Vec<Token>
}

impl<'slice> Tokenizer<'slice> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer { 
            source: source.as_bytes(),
            lexicon: RefCell::from(Lexicon::new(false)),
            source_tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self) {
        for token in self.source.iter() {
            self.lexicon.borrow_mut().advance(token.clone());
        }

        //TODO: Handle remaining values, or implement some kind of EOF function inside lexicon.
    }

    fn try_end_token(&mut self) {
        let token_match = self.lexicon.borrow_mut().end();
        match token_match { 
            LexiconMatch::Resolved(token) => {
                self.source_tokens.push(token.clone());
            },
            LexiconMatch::Unresolved() => {
                panic!("Could not resolve token [... insert token here]. {:?}", self.lexicon.borrow().get_value());
            },
            LexiconMatch::Unprocessed() => {
                panic!("Lexicon encountered an unaccounted for error.")
            }
        }
    }

    fn is_whitespace(&self, token: &u8) -> bool {
        let whitespace_tokens = vec![ 9, 10, 13, 32];
        return whitespace_tokens.contains(token);
    }    
}