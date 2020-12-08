use super::token::{TimeUnits, Token, TokenError, Weekdays};
use std::str;

pub trait Lexer {
    fn next_token(&mut self) -> Option<Token>;
}

pub trait LexerFactory {
    fn new_lexer(token: &str) -> dyn Lexer;
}

pub struct DatetokenLexer<'token> {
    token: &'token [u8],
    current_char: Option<&'token u8>,
    current_pos: usize,
    next_pos: usize,
}

fn is_digit(n: &u8) -> bool {
    return n >= &48 && n <= &57;
}

fn is_letter(n: &u8) -> bool {
    return (n >= &97 && n <= &122) || (n >= &65 && n <= &90);
}

impl<'token> DatetokenLexer<'token> {
    pub fn new(token: &'token str) -> Result<Self, TokenError> {
        let bts = token.trim().as_bytes();
        if bts.len() < 3 {
            return Err(TokenError);
        }
        let mut lexer = Self {
            token: bts,
            current_char: None,
            next_pos: 0,
            current_pos: 0,
        };
        lexer.read_char();
        Ok(lexer)
    }

    fn read_char(&mut self) {
        if self.next_pos < self.token.len() {
            self.current_char = Some(&self.token[self.next_pos]);
        } else {
            self.current_char = None;
        }
        self.current_pos = self.next_pos;
        self.next_pos += 1;
    }

    fn read_number(&mut self) -> Option<u16> {
        let pos = self.current_pos;
        loop {
            if let Some(ch) = self.current_char {
                if !is_digit(ch) {
                    break;
                }
                self.read_char();
            } else {
                break;
            }
        }

        if let Some(payload) = self.token.get(pos..self.current_pos) {
            if let Ok(number_slice) = str::from_utf8(payload) {
                if let Ok(number) = number_slice.parse::<u16>() {
                    return Some(number);
                }
            }
        }
        None
    }

    fn read_word(&mut self) -> Option<&str> {
        let pos = self.current_pos;
        loop {
            if let Some(ch) = self.current_char {
                if !is_letter(ch) {
                    break;
                }
                self.read_char();
            } else {
                break;
            }
        }
        if let Some(payload) = self.token.get(pos..self.current_pos) {
            if let Ok(slice) = str::from_utf8(payload) {
                return Some(slice);
            }
        }
        None
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let token = match self.current_char {
            Some(part) => match part {
                b'n' => match self.read_word() {
                    Some("now") => return Some(Token::Now),
                    _ => return Some(Token::Illegal(part.to_string())),
                },
                b'/' => Some(Token::SnapBegin),
                b'@' => Some(Token::SnapEnd),
                b'+' => Some(Token::Add),
                b'-' => Some(Token::Sub),
                _ => {
                    if is_digit(part) {
                        if let Some(n) = self.read_number() {
                            return Some(Token::Number(n));
                        }
                    }
                    if is_letter(part) {
                        if let Some(word) = self.read_word() {
                            if let Ok(t) = word.parse::<Token>() {
                                return Some(t);
                            } else {
                                return Some(Token::Illegal(word.to_string()));
                            }
                        }
                    }

                    None
                }
            },
            None => {
                if let Ok(payload) = str::from_utf8(&self.token) {
                    return Some(Token::Illegal(payload.to_string()));
                }
                None
            }
        };

        self.read_char();
        return token;
    }
}

//pub struct DatetokenLexerFactory;
//
//impl DatetokenLexerFactory {
//    pub fn new_lexer(token: &str) -> DatetokenLexer {
//        DatetokenLexer::new(token)
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexes_token() {
        match DatetokenLexer::new("now/w+1d-3h@M-9s") {
            Ok(mut lexer) => {
                let expected_tokens: &[Token] = &[
                    Token::Now,
                    Token::SnapBegin,
                    Token::Unit(TimeUnits::Week),
                    Token::Add,
                    Token::Number(1),
                    Token::Unit(TimeUnits::Day),
                    Token::Sub,
                    Token::Number(3),
                    Token::Unit(TimeUnits::Hour),
                    Token::SnapEnd,
                    Token::Unit(TimeUnits::Month),
                    Token::Sub,
                    Token::Number(9),
                    Token::Unit(TimeUnits::Sec),
                ];
                for expected_token in expected_tokens.iter() {
                    let next_token = lexer.next_token();
                    if next_token.is_some() {
                        assert_eq!(*expected_token, next_token.unwrap());
                    } else {
                        assert!(false, "unexpected token error, want {}", &expected_token);
                    }
                }
            }
            Err(err) => {
                assert_eq!(None, Some(err));
            }
        };
    }

    #[test]
    fn lexes_no_token() {
        let res = DatetokenLexer::new("");

        if res.is_ok() {
            assert_eq!(Some(TokenError), None)
        }
    }

    #[test]
    fn lexes_number() {
        match DatetokenLexer::new("123") {
            Ok(mut lexer) => {
                let next_token = lexer.next_token();
                if next_token.is_some() {
                    assert_eq!(Token::Number(123), next_token.unwrap());
                } else {
                    assert!(false, "unexpected token error, want Token::illegal");
                }
            }
            Err(err) => {
                assert_eq!(None, Some(err));
            }
        }
    }

    #[test]
    fn lexes_invalid_token_valid_begin() {
        match DatetokenLexer::new("/+-123") {
            Ok(mut lexer) => {
                let expected_tokens: &[Token] =
                    &[Token::SnapBegin, Token::Add, Token::Sub, Token::Number(123)];
                for expected_token in expected_tokens.iter() {
                    let next_token = lexer.next_token();
                    if next_token.is_some() {
                        assert_eq!(*expected_token, next_token.unwrap());
                    } else {
                        assert!(false, "unexpected token error, want {}", &expected_token);
                    }
                }
            }
            Err(err) => {
                assert_eq!(None, Some(err))
            }
        }
    }

    #[test]
    fn lexes_invalid_token() {
        match DatetokenLexer::new("asap/+-123") {
            Ok(mut lexer) => {
                let expected_tokens: &[Token] = &[
                    Token::Illegal("asap".to_string()),
                    Token::SnapBegin,
                    Token::Add,
                    Token::Sub,
                    Token::Number(123),
                ];
                for expected_token in expected_tokens.iter() {
                    let next_token = lexer.next_token();
                    if next_token.is_some() {
                        assert_eq!(*expected_token, next_token.unwrap());
                    } else {
                        assert!(false, "unexpected token error, want {}", &expected_token);
                    }
                }
            }
            Err(err) => {
                assert_eq!(None, Some(err));
            }
        }
    }
}
