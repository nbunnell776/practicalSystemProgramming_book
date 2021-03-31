/// This program reads tokens returned by Tokenizer and converts them into AST.
// Standard lib
use std::fmt;

//Internal modules
use ssuper::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

// Structs and constants

// Parser struct
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

// Public methods of Parser

impl<'a> Parser <'a> {
    // Create a new instance of Parser
    pub fn new(expr: &'a str) -> Result<Self, ParserError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }
    
    // Take an arithmetic expression as input and return AST
    
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
}

// Private methods of Parser

impl<'a> Parser<'a> {
    // Retrieve the next token from arithmetic expression and set it to current_token field in Parser struct
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match.self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }
    
    // Main workhorse method that is called recursively
    
    
}









