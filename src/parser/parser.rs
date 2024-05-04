use core::panic;

use crate::{
    ast::ast::{Identifier, LetStatement, Program, Statement},
    lexer::lexer::Lexer,
    token::token::{Token, TokenType, ASSIGN, EOF, IDENT, LET, SEMICOLON},
};

struct Parser {
    lexer: Box<Lexer>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer: Box::new(lexer),
            cur_token: None,
            peek_token: None,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token.as_ref().unwrap().r#type != EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                println!("Achou statement");
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        let current_token = match self.cur_token.as_ref() {
            Some(ct) => ct,
            None => panic!("cur_token is none"),
        };
        match current_token.r#type {
            LET => {
                println!("Parse let statement");
                Some(self.parse_let_statement()?)
            }
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<LetStatement>> {
        let first_token = self.cur_token.take();

        if !self.expect_peek(IDENT) {
            return None;
        }

        let identifier_token = self.cur_token.take().unwrap();
        let identifier = Identifier {
            token: identifier_token.clone(),
            value: identifier_token.literal.clone(),
        };

        if !self.expect_peek(ASSIGN) {
            return None;
        }

        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(LetStatement {
            token: first_token.unwrap(),
            name: Box::new(identifier),
        }))
    }

    fn cur_token_is(&self, token: TokenType) -> bool {
        match self.cur_token.as_ref() {
            Some(ct) => ct.r#type == token,
            None => false,
        }
    }

    fn peek_token_is(&self, token: TokenType) -> bool {
        match self.peek_token.as_ref() {
            Some(pt) => pt.r#type == token,
            None => false,
        }
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod ast_tests {
    use crate::ast::ast::{Node, Statement};

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got {}",
            program.statements.len()
        );
        let tests = vec![("x"), ("y"), ("foobar")];

        for (i, identifier) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            test_let_statement(stmt, identifier);
        }
    }

    fn test_let_statement(stmt: &Box<dyn Statement>, name: &str) -> bool {
        assert_eq!(
            stmt.token_literal(),
            "let",
            "statement token literal not 'let'. got {}",
            stmt.token_literal()
        );

        let let_stmt_option = stmt.as_let_statement();
        assert!(
            let_stmt_option.is_some(),
            "the statement is not a let statement"
        );
        let let_stmt = let_stmt_option.unwrap();

        assert_eq!(
            let_stmt.name.value, name,
            "let statement name value is not '{}'. got '{}'",
            name, let_stmt.name.value
        );

        assert_eq!(
            let_stmt.name.token_literal(),
            name,
            "let statement name is not '{}'. got '{}'",
            name,
            let_stmt.name.token_literal()
        );

        true
    }
}
