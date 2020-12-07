use crate::lexer::{DatetokenLexer, DatetokenLexerFactory, Lexer, LexerFactory};
use crate::parser::{DatetokenParser, Parser};
use std::time::Instant;

pub trait Evaluator {
    fn eval() -> Option<Instant>;
}

struct DatetokenEvaluator {
    lexer_factory: impl LexerFactory,
    parser_factory: impl LexerFactory,
}

impl DatetokenEvaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(token: &str) -> Option<Instant> {
        let lexer = DatetokenLexer::new(token);
        let parser = DatetokenParser::new(lexer);
        let evaluator = DatetokenEvaluator::new(parser);
        evaluator.eval()
    }
}
