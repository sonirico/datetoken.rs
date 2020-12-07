use crate::lexer::{DatetokenLexerFactory};

pub trait Parser {
    parse(&mut self) -> bool;
}

pub trait ParserFactory {
    new_parser(&self) -> impl Parser;
}


pub struct DatetokenParser {
    lexer_factory: impl lexer_factory
}


impl Parser for DatetokenParser {
    pub fn new() -> Self {
        Self {
            lexer: 
        }
    }

    pub fn parse(&mut self) -> bool {
        unimplemented!()
    }
}

pub struct DatetokenParserFactory {
    lexer_factory: impl DatetokenLexerFactory,
}

impl DatetokenParserFactory {
    pub fn new(lexer_factory: &DatetokenLexerFactory) -> Self {
        Self {
            lexer_factory
        }
    }
}

impl DatetokenParserFactory for DatetokenParserFactory {
    new_parser(&self) -> impl {
        DatetokenParser::new(self.lexer_factory)
    }
}