use crate::token::Token;
use crate::token::TokenMeaning;

/// Lexicon holds all the grammar for our language.
/// The Lexicon is namely used to match tokens that are constructed on a per character basis.
/// To use it:
/// - For every character call Lexicon::advance(character).
/// - Lexicon::advance will return wether we resolved it to one keyword, has possibilities or is a unknown token.
pub struct Lexicon {
    /// List of keywords that are available in our language.
    tokens: Vec<Token>,

    /// The current token we are trying to match to a keyword.
    match_value: Vec<u8>,

    /// All the possible keywords in our language given the current value of match_token.
    matches: Vec<Token>
}

///  LexiconMatch determines if a token has been found, can be found or was illegal.
///  If resolved, then we pass the token in that we found.
#[derive(Debug, Clone)]
pub enum LexiconMatch {

    /// Used to return a single token with matched one on one.
    Resolved(Token),

    /// Unresolved means that it failed to match anything within the lexicon, but still does has 1 or more potential matches.
    Unresolved(),

    // Used when the given characters does not match to anything and does not have any potential matches.
    Illegal(),

    // We matched nothing, we don't have any pending working to do.
    Nothing(),
}

impl Lexicon {
    pub fn new(tests: bool) -> Self {
        let mut lexicon = Lexicon {
            tokens: vec![
                Token::new("{", TokenMeaning::OpenScope),
                Token::new("}", TokenMeaning::CloseScope),

            ],
            match_value: Vec::new(),
            matches: Vec::<Token>::new()
        };

        if tests {
            lexicon.tokens.push(Token::new("findme", TokenMeaning::TestValue));
            lexicon.tokens.push(Token::new("findmetoo", TokenMeaning::TestValue));
            lexicon.tokens.push(Token::new("findmetoobutlonger", TokenMeaning::TestValue));

        }

        lexicon.reset();
        return lexicon;
    }

    pub fn reset(&mut self) {
        self.match_value.clear();
        self.matches = self.tokens.clone();
    }

    /// Consumes a new character and tries to match the current value.
    pub fn advance(&mut self, character: u8) -> LexiconMatch {
        //Don't process any whitespaces, as they have no meaning, this isn't python.
        if self.is_whitespace(&character) {
            return LexiconMatch::Unresolved();
        }

        self.match_value.push(character);
        self.filter_matches();

        //  All our matches are empty, meaning this was an illegal statement.
        if self.matches.len() == 0 {
            self.reset();
            return LexiconMatch::Illegal();
        }
        //  We only had one possibility, still need to check if this is still the exact same...
        else if self.matches.len() == 1 {
            let found_token = self.matches.first().unwrap().clone();

            if found_token.equals(&self.match_value) {
                self.reset();
                return LexiconMatch::Resolved(found_token);
            }
            else {
                return LexiconMatch::Unresolved();
            }
        }
        // We still have potential matches.
        else {
            return LexiconMatch::Unresolved();
        }
    }

    pub fn end_of_source(&self) -> LexiconMatch {
        //We have not pending values left to EOS it out.
        if self.match_value.len() == 0 {
            return LexiconMatch::Nothing();
        }

        //Loop over our matches, check if we have a direct match.
        if self.matches.len() > 0 {
            for potential_match in &self.matches {
                if potential_match.equals(&self.match_value) {
                    let found_token = potential_match.clone();
                    return LexiconMatch::Resolved(found_token);

                }
            }
        }

        return LexiconMatch::Illegal();
    }

    fn filter_matches(&mut self) {
        let mut new_matches = Vec::<Token>::new();

        for token in self.matches.iter() {

            //If we have "bool" as a keyword and "bools" as current value...
            // Then the length will differ and it could never be this value.
            if self.match_value.len() > token.word.len() {
                continue;
            }

            let word = self.match_value.as_slice();

            //If we have "bignumber" as a keyword and "bin" as current value...
            // Then we could never match if the word to "bignumber".
            if !token.starts_with(word) {
                continue;
            }

            new_matches.push(token.clone());
        }

        self.matches = new_matches;
    }

    fn is_whitespace(&self, token: &u8) -> bool {
        let whitespace_tokens = vec![ 9, 10, 13, 32];
        return whitespace_tokens.contains(token);
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexicon_match_scope() {
        let mut lexicon = Lexicon::new(true);
        let source = "{".as_bytes();

        let result = lexicon.advance(source[0]);
        assert_eq!(true, matches!(result, LexiconMatch::Resolved(_)));
    }

    #[test]
    fn lexicon_match_scope_open_and_close() {
        let mut lexicon = Lexicon::new(true);
        let source = "{}".as_bytes();

        let mut result = lexicon.advance(source[0]);

        if let LexiconMatch::Resolved(token) = &result {
            assert_eq!(true, matches!(token.state, TokenMeaning::OpenScope));
        }
        else {
            assert!(false, "Result 1 was not resolved.")
        }

        result = lexicon.advance(source[1]);

        if let LexiconMatch::Resolved(token) = &result {
            assert_eq!(true, matches!(token.state, TokenMeaning::CloseScope));
        }
        else {
            assert!(false, "Result 2 was not resolved.")
        }
    }

    #[test]
    fn lexicon_match_scope_open_and_close_with_whitspace() {
        let mut lexicon = Lexicon::new(true);
        let source = "{ }".as_bytes();

        let mut result = lexicon.advance(source[0]);
        if let LexiconMatch::Resolved(token) = &result {
            assert_eq!(true, matches!(token.state, TokenMeaning::OpenScope));
        }
        else {
            assert!(false, "Result 1 was not resolved.")
        }

        result = lexicon.advance(source[1]);
        assert_eq!(true, matches!(result, LexiconMatch::Unresolved()));

    
        result = lexicon.advance(source[2]);
        if let LexiconMatch::Resolved(token) = &result {
            assert_eq!(true, matches!(token.state, TokenMeaning::CloseScope));
        }
        else {
            assert!(false, "Result 3 was not resolved.")
        }

    }

    #[test]
    fn lexicon_can_manage_eos() {
        let mut lexicon = Lexicon::new(true);
        let source = "findme".as_bytes();
        let mut result = LexiconMatch::Illegal();

        for i in 0..6 {
            result = lexicon.advance(source[i]);
            assert_eq!(true, matches!(result, LexiconMatch::Unresolved()));  
        }
        
        result = lexicon.end_of_source();
        assert_eq!(true, matches!(result, LexiconMatch::Resolved(_)));  
    }

    #[test]
    fn lexicon_match_multi_character_keyword() {
        let mut lexicon = Lexicon::new(true);
        let source = "findme".as_bytes();
        let mut result = LexiconMatch::Illegal();

        for i in 0..source.len() {
            result = lexicon.advance(source[i]);
            assert_eq!(true, matches!(result, LexiconMatch::Unresolved()));  
        }
        
        result = lexicon.end_of_source();
        assert_eq!(true, matches!(result, LexiconMatch::Resolved(_)));
    }

    #[test]
    fn lexicon_can_match_ambiguous_matches_shortest() {
        let mut lexicon = Lexicon::new(true);
        let source = "findme".as_bytes();
        let mut result = LexiconMatch::Illegal();

        for i in 0..source.len() {
            result = lexicon.advance(source[i]);
            assert_eq!(true, matches!(result, LexiconMatch::Unresolved()));  
        }
        
        result = lexicon.end_of_source();
        if let LexiconMatch::Resolved(token) = &result {
            assert_eq!(true, token.equals("findme".as_bytes()));
        }
        else {
            assert!(false, "Result was not resolved.")
        };
    }

    #[test]
    fn lexicon_can_match_ambiguous_matches_longest() {
        let mut lexicon = Lexicon::new(true);
        let source = "findmetoo".as_bytes();
        let mut result = LexiconMatch::Illegal();

        for i in 0..source.len() {
            result = lexicon.advance(source[i]);
            assert_eq!(true, matches!(result, LexiconMatch::Unresolved()));  
        }
        
        result = lexicon.end_of_source();
        if let LexiconMatch::Resolved(token) = &result {
            assert_eq!(true, token.equals("findmetoo".as_bytes()));
        }
        else {
            assert!(false, "Result was not resolved.")
        };
    }

    
    #[test]
    fn lexicon_can_match_ambiguous_matches_nothing() {
        let mut lexicon = Lexicon::new(true);
        let source = "findmet".as_bytes();
        let mut result = LexiconMatch::Illegal();

        for i in 0..source.len() {
            result = lexicon.advance(source[i]);
            assert_eq!(true, matches!(result, LexiconMatch::Unresolved()));  
        }
        
        result = lexicon.end_of_source();
        assert_eq!(true, matches!(result, LexiconMatch::Illegal()));  
    }
}