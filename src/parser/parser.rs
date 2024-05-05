use core::panic;
use std::collections::HashMap;

use crate::{
    ast::ast::{
        Expression, ExpressionStatement, Identifier, LetStatement, Program, ReturnStatement,
        Statement,
    },
    lexer::lexer::Lexer,
    token::token::{Token, TokenType, ASSIGN, EOF, IDENT, LET, RETURN, SEMICOLON},
};

type PrefixParseFn = fn(&mut Parser) -> Box<dyn Expression>;
type InfixParseFn = fn(&mut Parser, dyn Expression) -> Box<dyn Expression>;

const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;

struct Parser {
    lexer: Box<Lexer>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer: Box::new(lexer),
            cur_token: None,
            peek_token: None,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.next_token();
        parser.next_token();

        parser.register_prefix(IDENT, Parser::parse_identifier);

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while !self.cur_token_is(EOF) {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.expect("statement is none"));
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
            LET => Some(self.parse_let_statement()?),
            RETURN => Some(self.parse_return_statement()?),
            _ => Some(self.parse_expression_statement()?),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<LetStatement>> {
        let let_token = match self.cur_token.take() {
            Some(ct) => {
                if ct.r#type != LET {
                    return None;
                }
                ct
            }
            None => return None,
        };

        if !self.expect_peek(IDENT) {
            return None;
        }
        let identifier_token = match self.cur_token.clone() {
            Some(ct) => ct,
            None => return None,
        };
        let identifier = Identifier {
            token: identifier_token.clone(),
            value: identifier_token.literal,
        };

        if !self.expect_peek(ASSIGN) {
            return None;
        }

        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(LetStatement {
            token: let_token,
            name: Box::new(identifier),
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<ReturnStatement>> {
        let return_token = match self.cur_token.take() {
            Some(ct) => {
                if ct.r#type != RETURN {
                    return None;
                }
                ct
            }
            None => return None,
        };

        self.next_token();

        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ReturnStatement {
            token: return_token,
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<ExpressionStatement>> {
        let token = match self.cur_token.as_ref() {
            Some(ct) => ct.clone(),
            None => return None,
        };

        let expression = match self.parse_expression(LOWEST) {
            Some(pe) => pe,
            None => return None,
        };

        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ExpressionStatement { token, expression }))
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
            self.peek_error(token);
            false
        }
    }

    fn peek_error(&mut self, token: TokenType) {
        let msg = format!(
            "expected next token to be {}, got {}",
            token,
            self.peek_token
                .as_ref()
                .expect("peek token should not be None")
                .r#type
        );
        self.errors.push(msg);
    }

    fn register_prefix(&mut self, token_type: TokenType, fun: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, fun);
    }

    fn register_infix(&mut self, token_type: TokenType, fun: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, fun);
    }

    fn parse_expression(&mut self, precedence: u8) -> Option<Box<dyn Expression>> {
        let cur_type = &self
            .cur_token
            .as_ref()
            .expect("cur token should not be None")
            .r#type;
        if !self.prefix_parse_fns.contains_key(cur_type) {
            return None;
        }
        let prefix = self.prefix_parse_fns[cur_type];

        let left_exp = prefix(self);

        Some(left_exp)
    }

    fn parse_identifier(&mut self) -> Box<dyn Expression> {
        let identifier = match self.cur_token.clone() {
            Some(ct) => ct,
            None => panic!("invalid!"),
        };
        Box::new(Identifier {
            token: identifier.clone(),
            value: identifier.literal,
        })
    }
}

#[cfg(test)]
mod parser_tests {
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
        check_parser_errors(&parser);
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

    #[test]
    fn test_return_statements() {
        let input = r"
        return 5;
        return 10;
        return 993322;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            3,
            "program statements does not contain 3 statements. got {}",
            program.statements.len()
        );

        for stmt in program.statements {
            let return_stmt_option = stmt.as_return_statement();
            assert!(
                return_stmt_option.is_some(),
                "the statement is not a return statement"
            );
            let return_stmt = return_stmt_option.unwrap();
            assert_eq!(
                return_stmt.token_literal(),
                "return",
                "return stmt token literal it not 'return'. got {}",
                return_stmt.token_literal()
            )
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = r"foobar;";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got {}",
            program.statements.len()
        );

        let expression_stmt_option = program.statements[0].as_expression_statement();
        assert!(
            expression_stmt_option.is_some(),
            "the statement is not an expression statement"
        );
        let expression_stmt = expression_stmt_option.unwrap();

        let identifier_expression_option = expression_stmt.expression.as_identifier_expression();
        assert!(
            identifier_expression_option.is_some(),
            "the expression is not an identifier expression"
        );
        let identifier_expression = identifier_expression_option.unwrap();
        assert_eq!(
            identifier_expression.value, "foobar",
            "identifier expression value is not {}. got {}",
            "foobar", identifier_expression.value
        );
        assert_eq!(
            identifier_expression.token_literal(),
            "foobar",
            "identifier expression token literal is not {}. got {}",
            "foobar",
            identifier_expression.token_literal()
        );
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

    fn check_parser_errors(parser: &Parser) {
        let errors = &parser.errors;

        if errors.len() == 0 {
            return;
        }

        let mut error_message = String::new();
        for error in errors {
            error_message.push_str(format!("{}\n", error).as_str());
        }

        assert!(
            errors.len() == 0,
            "parser has {} errors: {}",
            errors.len(),
            error_message
        );
    }
}
