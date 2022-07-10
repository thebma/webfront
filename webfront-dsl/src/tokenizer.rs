use std::str;
use std::cell::RefCell;

use crate::lexicon::Lexicon;
use crate::lexicon::LexiconMatch;

use crate::token::Token;

//TODO AB 03-07-2022 Support UTF-8 and UTF-16 and test if it gets parsed properly.
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

    //TODO AB 03-07-2022, horrible solution, fix these "borrow(_mut)"'s. Right now don't have enough Rust knowledge to iron this out.
    pub fn tokenize(&mut self) {
        for token in self.source.iter() {
            if self.is_whitespace(token) {
                continue;
            }

            
            self.lexicon.borrow_mut().advance(token.clone());

            //
            //  Hitting a whitespace can mean two things:
            //  a) We are starting to parse a new token, we can ignore it until we hit a non-white space.
            //  or b) 
            //
            if self.is_whitespace(token) {
                if self.lexicon.borrow().has_value() {
                    self.try_end_token();
                }
            } else {
                let _exact = self.lexicon.borrow_mut().advance(token.clone());

                

                println!("{:?}", token.clone())
            }
        }

        //
        // Handle any remaining content in the lexicon.
        // Files don't always nicely end with a whitespace.
        //
        if self.lexicon.borrow().has_value() {
            self.try_end_token();
        }
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