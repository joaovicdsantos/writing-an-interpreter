use crate::token::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn as_let_statement(&self) -> Option<&LetStatement> {
        None
    }
    fn as_return_statement(&self) -> Option<&ReturnStatement> {
        None
    }
    fn as_expression_statement(&self) -> Option<&ExpressionStatement> {
        None
    }
}

pub trait Expression: Node {
    fn expression_node(&self);
    fn as_identifier_expression(&self) -> Option<&Identifier> {
        None
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            &""
        }
    }
    fn string(&self) -> String {
        let mut out = String::new();
        for statement in self.statements.iter() {
            out.push_str(&statement.string())
        }
        out
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    // pub value: dyn Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(format!("{} ", self.token_literal()).as_str());
        out.push_str(&self.name.string());
        out.push_str(" = ");
        // add value here
        out.push_str(";");
        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn as_let_statement(&self) -> Option<&LetStatement> {
        Some(&self)
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Statement for Identifier {
    fn statement_node(&self) {}
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    fn as_identifier_expression(&self) -> Option<&Identifier> {
        Some(&self)
    }
}

pub struct ReturnStatement {
    pub token: Token,
    // return_value: dyn Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(format!("{} ", self.token_literal()).as_str());
        // add value here
        out.push_str(";");
        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn as_return_statement(&self) -> Option<&ReturnStatement> {
        Some(&self)
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        self.expression.string()
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    fn as_expression_statement(&self) -> Option<&ExpressionStatement> {
        Some(&self)
    }
}

#[cfg(test)]
mod ast_tests {
    use crate::token::token::{IDENT, LET};

    use super::*;

    #[test]
    fn test_string() {
        let program = &Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    r#type: LET,
                    literal: "let".to_string(),
                },
                name: Box::new(Identifier {
                    token: Token {
                        r#type: IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                }),
            })],
        };

        assert_eq!(
            program.string(),
            "let myVar = ;",
            "program string wrong. got {}",
            program.string()
        );
    }
}
