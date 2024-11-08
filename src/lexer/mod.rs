use std::collections::HashMap;
use regex;

/// Lexer which processes source and returns a vec of tokens
#[derive(Debug)]
struct Lexer {
    /// Source code
    source: String,
    /// Processed Tokens
    tokens: Vec<Token>,
    /// Token Regexes
    token_regex: HashMap<TokenType, regex::Regex>,
    /// Current Match Lengths
    match_lengths: HashMap<TokenType, usize>,
    /// Current position in the source code
    position: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let mut token_regex: HashMap<TokenType, regex::Regex> = HashMap::new();
        let mut match_lengths: HashMap<TokenType, usize> = HashMap::new();
        let tokens: Vec<Token> = Vec::new();
        // Add needed regexes
        // Note: Will only be searching at the start of a string, since this will process
        // Through the source code removing each token as it is found
        token_regex.insert(TokenType::Identifier, regex::Regex::new(r"^[a-zA-Z_]\w*\b").unwrap());
        match_lengths.insert(TokenType::Identifier, 0);
        token_regex.insert(TokenType::Constant, regex::Regex::new(r"^[0-9]+\b").unwrap());
        match_lengths.insert(TokenType::Constant, 0);
        // Keywords will just be matched as identifiers, and then further identified later
        token_regex.insert(TokenType::LeftParen, regex::Regex::new(r"^\(").unwrap());
        match_lengths.insert(TokenType::LeftParen, 0);
        token_regex.insert(TokenType::RightParen, regex::Regex::new(r"^\)").unwrap());
        match_lengths.insert(TokenType::RightParen, 0);
        token_regex.insert(TokenType::LeftBrace, regex::Regex::new(r"^\{").unwrap());
        match_lengths.insert(TokenType::LeftBrace, 0);
        token_regex.insert(TokenType::RightBrace, regex::Regex::new(r"^}").unwrap());
        match_lengths.insert(TokenType::RightBrace, 0);
        token_regex.insert(TokenType::Semicolon, regex::Regex::new(r"^;").unwrap());
        match_lengths.insert(TokenType::Semicolon, 0);
        // Return lexer object
        Lexer{
            source, tokens, token_regex, match_lengths, position:0
        }
    }

    fn zero_match_lengths(&mut self){
        for v in self.match_lengths.values_mut(){
            *v = 0;
        }
    }

    fn skip_whitespace(&mut self){
        let bytes = self.source.as_bytes();
        while self.position < self.source.len() && (bytes[self.position] as char).is_whitespace(){
            self.position += 1;
        }
    }

    /// Find the max match length for TokenType in match_lengths
    fn find_max_token(&self) -> Result<TokenType, LexerError> {
        let max_key = match self.match_lengths.iter().max_by_key(|&(_, v)| v){
            None => {return Err(LexerError::UnknownToken)}
            Some(v) => {v}
        };
        Ok(max_key.0.clone())
    }

    /// Processes source code into series of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        // While there is still sourcecode to consume, do so
        while self.position < self.source.len() {
            // Zero all match lengths
            self.zero_match_lengths();
            // Remove any leading whitespace from the string
            self.skip_whitespace();
            // Iterate through all possible token types, finding the match lengths
            for (&token_type, regex) in &self.token_regex {
                // If there is a regex match
                if let Some(re_match) = regex.find(&self.source[self.position..]) {
                    self.match_lengths.insert(token_type, re_match.len());
                }
            }
            // Find which token type is being scanned
            match self.find_max_token()? {
                TokenType::Identifier => {
                    // The match can either be a identifier, or a keyword
                    // So find the match, check if it matches keywords, then
                    // Generate an appropriate token, and remove the match from
                    // the source
                    let re_match = match self.token_regex[&TokenType::Identifier].find(&self.source[self.position..]){
                        None => {return Err(LexerError::RegexFailure)},
                        Some(re_match) => re_match,
                    };
                    match re_match.as_str() {
                        "int"=>{
                            self.tokens.push(Token::new_int());
                            self.position += re_match.len();
                        }
                        "void"=>{
                            self.tokens.push(Token::new_void());
                            self.position += re_match.len();
                        }
                        "return"=>{
                            self.tokens.push(Token::new_return());
                            self.position += re_match.len();
                        }
                        identifier=>{
                            // Didn't match any keywords, it is an identifier
                            self.tokens.push(Token::new_identifier(identifier.to_string()));
                            self.position += re_match.len();
                        }
                    }
                }
                TokenType::Constant => {
                    let re_match = match self.token_regex[&TokenType::Constant].find(&self.source[self.position..]) {
                        Some(re_match) => re_match,
                        None=>{return Err(LexerError::UnknownToken)}
                    };
                    self.tokens.push(Token::new_constant(re_match.as_str().to_string()));
                    self.position += re_match.len();
                }
                TokenType::Int | TokenType::Void | TokenType::Return => {} //Can't actually happen
                TokenType::LeftParen => {
                    self.tokens.push(Token::new_left_paren());
                    self.position += 1;
                }
                TokenType::RightParen => {
                    self.tokens.push(Token::new_right_paren());
                    self.position += 1;
                }
                TokenType::LeftBrace => {
                    self.tokens.push(Token::new_left_brace());
                    self.position += 1;
                }
                TokenType::RightBrace => {
                    self.tokens.push(Token::new_right_brace());
                    self.position += 1;
                }
                TokenType::Semicolon => {
                    self.tokens.push(Token::new_semicolon());
                    self.position += 1;
                }
            }
        }

        Ok(self.tokens.clone())
    }
}


#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) enum TokenType {
    Identifier,
    Constant,
    Int,
    Void,
    Return,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: Option<String>,
}

impl Token {
    fn new_identifier(lexeme: String) -> Token {
        Token{token_type: TokenType::Identifier , lexeme: Some(lexeme)}
    }

    fn new_constant(lexeme: String) -> Token {
        Token{token_type: TokenType::Constant, lexeme: Some(lexeme)}
    }

    fn new_int() -> Token {
        Token {token_type: TokenType::Int, lexeme: None, }
    }

    fn new_void() -> Token {
        Token {token_type: TokenType::Void, lexeme: None, }
    }

    fn new_return() -> Token {
        Token {token_type: TokenType::Return, lexeme: None, }
    }

    fn new_left_paren() -> Token {
        Token {token_type: TokenType::LeftParen, lexeme: None, }
    }

    fn new_right_paren() -> Token {
        Token {token_type: TokenType::RightParen, lexeme: None, }
    }

    fn new_left_brace() -> Token {
        Token {token_type: TokenType::LeftBrace, lexeme: None, }
    }

    fn new_right_brace() -> Token {
        Token {token_type: TokenType::RightBrace, lexeme: None, }
    }

    fn new_semicolon() -> Token {
        Token {token_type: TokenType::Semicolon, lexeme: None, }
    }

}

#[derive(Debug)]
enum LexerError {
    RegexFailure,
    UnknownToken, 
    MatchLengthError,
}

#[cfg(test)]
mod test_lexer {
    use super::*;
    #[test]
    fn test_create() {
        _ = Lexer::new("int main(void) {return 0;}".to_string());
    }
    #[test]
    fn test_tokenize() {
        let mut lexer = Lexer::new("int main(void) {return 0;}".to_string());
        let tokens = lexer.tokenize();
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        let expected_tokens = vec![Token::new_int(), Token::new_identifier("main".to_string()),
                                   Token::new_left_paren(), Token::new_void(), Token::new_right_paren(),Token::new_left_brace(),
                                   Token::new_return(), Token::new_constant("0".to_string()),
                                   Token::new_semicolon(), Token::new_right_brace()];
        assert_eq!(tokens.len(),expected_tokens.len());
        assert!(tokens.iter().zip(expected_tokens).all(|(a, b)| *a == b));
    }
}
