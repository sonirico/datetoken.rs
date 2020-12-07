use super::token::Token;

pub trait Lexer {
    fn next_token(&mut self) -> Option<Token>;
}

pub trait LexerFactory {
    fn new_lexer(token: &str) -> dyn Lexer;
}

pub struct DatetokenLexer<'token> {
    token: &'token str,
    current_pos: usize,
    next_pos: usize,
}

impl<'token> DatetokenLexer<'token> {
    pub fn new(token: &'token str) -> Self {
        Self {
            token,
            current_pos: 0,
            next_pos: 0,
        }
    }

    fn read_number(&mut self) -> Option<u16> {
        let pos = self.current_pos;
        match &self.token.get(pos..) {
            Some(sub_slice) => {
                let mut end = sub_slice.len();
                for (i, ch) in sub_slice.chars().enumerate() {
                    if !ch.is_numeric() {
                        end = i;
                        break;
                    }
                }
                if let Ok(number) = &sub_slice[pos..end].parse::<u16>() {
                    Some(*number)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn read_word(&mut self) -> Option<&str> {
        let pos = self.current_pos;
        match &self.token.get(pos..) {
            Some(sub_slice) => {
                for (i, ch) in sub_slice.chars().enumerate() {
                    if !ch.is_ascii_alphabetic() {
                        return Some(&sub_slice[pos..i]);
                    }
                }
                Some(&sub_slice[..])
            }
            None => None,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos >= self.token.len() {
            return None;
        }
        let pos = self.current_pos;
        let next_pos = self.current_pos + 1;
        match self.token.get(pos..next_pos) {
            Some(part) => match part {
                "n" => match self.read_word() {
                    Some("now") => Some(Token::Now),
                    _ => None,
                },
                _ => match part.chars().next() {
                    Some(ch) => {
                        if ch.is_numeric() {
                            return match self.read_number() {
                                Some(n) => Some(Token::Number(n)),
                                None => Some(Token::Illegal(part.to_string())),
                            };
                        }
                        None
                    }
                    None => None,
                },
            },
            None => None,
        }
    }
}

pub struct DatetokenLexerFactory;

impl DatetokenLexerFactory {
    pub fn new_lexer(token: &str) -> DatetokenLexer {
        DatetokenLexer::new(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexes_now_token() {
        let mut lexer = DatetokenLexer::new("now");
        let expected_tokens: [Token; 1] = [Token::Now];
        for expected_token in &expected_tokens {
            let next_token = lexer.next_token();
            dbg!(&expected_token);
            dbg!(&next_token);
            if next_token.is_some() {
                assert_eq!(*expected_token, next_token.unwrap());
            } else {
                assert!(false, "unexpected token error, want {}", &expected_token);
            }
        }
        //assert_eq!(2 + 2, 4);
    }
}
