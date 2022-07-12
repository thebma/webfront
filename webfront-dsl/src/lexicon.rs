use crate::token::Token;
use crate::token::TokenMeaning;

///
/// Lexicon holds all the grammar for our language.
/// The Lexicon is namely used to match tokens that are constructed on a per character basis.
/// To use it:
/// - For every character call Lexicon::advance(character).
/// - Lexicon::advance will return wether we resolved it to one keyword, has possibilities or is a unknown token.
pub struct Lexicon {
    ///
    /// List of keywords that are available in our language.
    tokens: Vec<Token>,

    ///
    /// The current token we are trying to match to a keyword.
    match_value: Vec<u8>,

    ///
    /// All the possible keywords in our language given the current value of match_token.
    matches: Vec<Token>
}

///
///  LexiconMatch determines if a token has been found, can be found or was illegal.
///  If resolved, then we pass the token in that we found.
#[derive(Debug, Clone)]
pub enum LexiconMatch {

    /// Resolved to an unique token within the lexicon.
    Resolved(Token),

    /// Unresolved means that it failed to match anything within the lexicon. Making the subsequent statement invalid.
    Unresolved(),

    /// Lexicon did not know what to do, has left it unprocessed... which usually is a side effect of errorneous behaviour.
    Unprocessed()
}

//TODO: AB 03-07-22 these "meanings" need to be loaded from file, instead having a massive list in code.
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
            lexicon.tokens.push(Token::new("parse", TokenMeaning::TestValue));
            lexicon.tokens.push(Token::new("ambig", TokenMeaning::TestValue));
            lexicon.tokens.push(Token::new("ambiguous", TokenMeaning::TestValue));
        }

        lexicon.reset();
        return lexicon;
    }

    pub fn has_value(&self) -> bool {
        return self.match_value.len() >  0;
    }

    pub fn get_value(&self) -> Vec<u8> {
        return self.match_value.clone();
    }

    pub fn reset(&mut self) {
        self.match_value.clear();
        self.matches = self.tokens.clone();
    }

    /// Consumes a new character and tries to match the current value.
    pub fn advance(&mut self, character: u8) -> LexiconMatch {
        self.match_value.push(character);
        self.filter_matches();

        let mut result_match = LexiconMatch::Unresolved();

        //
        //  All our matches are empty, meaning this was an illegal statement.
        if self.matches.len() == 0 {
            result_match = LexiconMatch::Unresolved();
        }
        //
        //  We had an exact match.
        else if self.matches.len() == 1 {
            result_match = LexiconMatch::Resolved(self.matches.first().unwrap().clone())
        }
        else {
            //Ambiguous match!
            result_match = LexiconMatch::Unprocessed();
        }


        self.reset();
        return result_match;
    }


    fn filter_matches(&mut self) {
        let mut new_matches = Vec::<Token>::new();

        for token in self.matches.iter() {
            //
            // If we're matching "bool" and the current value is "boole"...
            //   token.word.len() = "bool", 4 characters
            //   self.match_value.len() = "boole", 5 characters
            // Then we could never match because token.word is smaller than what we currently have.
            if self.match_value.len() > token.word.len() {
                continue;
            }

            //
            //  If we have the following situation:
            //    token.word = "int"
            //    self.match_value = "it"
            //  The token.word does not start_with "it", there for it could never match.
            let word = self.match_value.as_slice();

            if !token.starts_with(word) {
                continue;
            }

            //
            //  Token is still eligible to be matched to something.
            //  We want to copy this value in our new vector with matches.
            new_matches.push(token.clone());
        }

        self.matches = new_matches;
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    // fn setup_lexicon(source: &str) -> Vec<LexiconMatch> {
    //     let mut lexicon = Lexicon::new(true);
    //     let mut match_results = Vec::<LexiconMatch>::new();

    //     lexicon.reset();

    //     for word_byte in source.bytes() {
    //         let lexicon_match = lexicon.advance(word_byte);

    //         if let LexiconMatch::Resolved(_) = lexicon_match {
    //            match_results.push(lexicon_match.clone());
    //         }
    //     }

    //     return match_results;
    // }

    // #[test]
    // fn lexicon_can_match_scope() {
    //     let token_match = setup_lexicon("{");

    //     assert_eq!(true, matches!(token_match, LexiconMatch::Resolved(_)));

    //     if let LexiconMatch::Resolved(token) = &token_match {
    //         assert_eq!("{", token.word.as_str());
    //         assert_eq!(true, matches!(token.state, TokenMeaning::OpenScope));
    //     } else {
    //         assert_eq!(true, false);
    //     }
    // }

    // #[test]
    // fn lexicon_can_resolve_multiple_characters() {
    //     let token_match = setup_lexicon("parse");

    //     assert_eq!(true, matches!(token_match, LexiconMatch::Resolved(_)));

    //     if let LexiconMatch::Resolved(token) = &token_match {
    //         assert_eq!("parse", token.word.as_str());
    //     }
    // }

    // #[test]
    // fn lexicon_resolve_with_ambiguous_result_with_lowest_exact_match() {
    //     let token_match = setup_lexicon("ambig");

    //     assert_eq!(true, matches!(token_match, LexiconMatch::Resolved(_)));

    //     if let LexiconMatch::Resolved(token) = &token_match {
    //         assert_eq!("ambig", token.word.as_str());
    //     }
    // }
    
    // #[test]
    // fn lexicon_resolve_with_ambiguous_result_with_highest_exact_match() {
    //     let token_match = setup_lexicon("ambiguous");

    //     assert_eq!(true, matches!(token_match, LexiconMatch::Resolved(_)));

    //     if let LexiconMatch::Resolved(token) = &token_match {
    //         assert_eq!("ambiguous", token.word.as_str());
    //     }
    // }

    // #[test]
    // fn lexicon_resolve_with_ambiguous_result_with_no_match() {
    //     let token_match = setup_lexicon("ambigu");

    //     assert_eq!(true, matches!(token_match, LexiconMatch::Unresolved()));
    // }


}