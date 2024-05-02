use crate::token::token::*;

pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position as usize >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peak_char(&self) -> char {
        if self.read_position as usize >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position as usize).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => {
                if self.peak_char() == '=' {
                    let fch = self.ch;
                    self.read_char();
                    Token {
                        r#type: EQ,
                        literal: format!("{}{}", fch, self.ch),
                    }
                } else {
                    Lexer::new_token(ASSIGN, self.ch)
                }
            }
            '+' => Lexer::new_token(PLUS, self.ch),
            '-' => Lexer::new_token(MINUS, self.ch),
            '!' => {
                if self.peak_char() == '=' {
                    let fch = self.ch;
                    self.read_char();
                    Token {
                        r#type: NEQ,
                        literal: format!("{}{}", fch, self.ch),
                    }
                } else {
                    Lexer::new_token(BANG, self.ch)
                }
            }
            '*' => Lexer::new_token(ASTERISK, self.ch),
            '/' => Lexer::new_token(SLASH, self.ch),
            '<' => Lexer::new_token(LT, self.ch),
            '>' => Lexer::new_token(GT, self.ch),
            ';' => Lexer::new_token(SEMICOLON, self.ch),
            '(' => Lexer::new_token(LPAREN, self.ch),
            ')' => Lexer::new_token(RPAREN, self.ch),
            '{' => Lexer::new_token(LBRACE, self.ch),
            '}' => Lexer::new_token(RBRACE, self.ch),
            ',' => Lexer::new_token(COMMA, self.ch),
            '\0' => Lexer::new_token(EOF, self.ch),
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token {
                        r#type: lookup_ident(literal.clone()),
                        literal,
                    };
                } else if self.ch.is_numeric() {
                    let literal = self.read_number();
                    return Token {
                        r#type: INT,
                        literal,
                    };
                }
                Lexer::new_token(ILLEGAL, self.ch)
            }
        };
        self.read_char();
        return tok;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }
        return self.input[position as usize..self.position as usize].to_string();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        return self.input[position as usize..self.position as usize].to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn new_token(token_type: TokenType, ch: char) -> Token {
        let literal = if ch == '\0' {
            "".to_string()
        } else {
            ch.to_string()
        };
        Token {
            r#type: token_type,
            literal,
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
}

#[cfg(test)]
mod lexer_tests {

    use super::*;

    #[test]
    fn test_next_token() {
        let input = r"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ";
        let tests = vec![
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INT, "5"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "result"),
            (ASSIGN, "="),
            (IDENT, "add"),
            (LPAREN, "("),
            (IDENT, "five"),
            (COMMA, ","),
            (IDENT, "ten"),
            (RPAREN, ")"),
            (SEMICOLON, ";"),
            (BANG, "!"),
            (MINUS, "-"),
            (SLASH, "/"),
            (ASTERISK, "*"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (GT, ">"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (IF, "if"),
            (LPAREN, "("),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (INT, "10"),
            (EQ, "=="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (INT, "10"),
            (NEQ, "!="),
            (INT, "9"),
            (SEMICOLON, ";"),
            (EOF, ""),
        ];
        let mut l = Lexer::new(input.to_string());
        for (expected_type, expected_literal) in tests {
            let tok = l.next_token();
            assert_eq!(tok.r#type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
