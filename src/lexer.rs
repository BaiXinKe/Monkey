use crate::token::{self, Token, TokenType};

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        let mut result = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        // store the first char to the Lexer
        result.read_char();
        result
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .unwrap_or_else(|| panic!("Failed to load item in: {}", self.read_position));
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    //Function set: Judge the char type using the char_type_helper function
    fn character_identifying<Func>(&mut self, pred: Func) -> String
    where
        Func: Fn(char) -> bool,
    {
        let position = self.position;
        while pred(self.ch) {
            self.read_char()
        }

        self.input[position..self.position].to_string()
    }

    fn is_letter(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn read_identifier(&mut self) -> String {
        self.character_identifying(Self::is_letter)
    }

    fn read_number(&mut self) -> String {
        self.character_identifying(Self::is_digit)
    }
    // ---------------------------------------------------------------------------

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or('\0')
    }

    fn consume_two_chars(&mut self, token_type: TokenType, literal: &str) -> Token {
        self.read_char();
        self.read_char();
        Token::new(token_type, literal)
    }

    pub(crate) fn next_token(&mut self) -> Token {
        use token::TokenType::*;

        self.skip_whitespace();
        let literal = self.ch.to_string();

        let tok_type = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    return self.consume_two_chars(Eq, "==");
                }
                Assign
            }
            ';' => Semicolon,
            '(' => Lparen,
            ')' => Rparen,
            ',' => Comma,
            '+' => Plus,
            '-' => Minus,
            '!' => {
                if self.peek_char() == '=' {
                    return self.consume_two_chars(NotEq, "!=");
                }
                Bang
            }
            '/' => Slash,
            '*' => Asterisk,
            '<' => Lt,
            '>' => Gt,
            '{' => Lbrace,
            '}' => Rbrace,
            '\0' => Eof,
            _ => {
                if Self::is_letter(self.ch) {
                    let ident = &self.read_identifier();
                    return Token::new(Token::lookup_ident(ident), ident);
                } else if Self::is_digit(self.ch) {
                    return Token::new(Int, &self.read_number());
                } else {
                    return Token::new(Illegal, std::mem::take(&mut self.ch).to_string().as_str());
                }
            }
        };

        self.read_char();
        Token::new(tok_type, &literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    fn helper_iter_item(expect_items: &[Token], lexer: &mut Lexer) {
        for (index, expect_token) in expect_items.iter().enumerate() {
            let actual_token = lexer.next_token();

            assert_eq!(
                actual_token.typee,
                expect_token.typee,
                "{}",
                format_args!(
                    "tests[{index}] - tokentype wrong. expected {}, got {}",
                    expect_token.typee, actual_token.typee
                )
            );

            assert_eq!(
                actual_token.literal,
                expect_token.literal,
                "{}",
                format_args!(
                    "tests[{index}] - literal wrong. expected {}, got {}",
                    expect_token.literal, actual_token.literal
                )
            )
        }
    }

    #[test]
    fn next_token() {
        let input = r#"=+(){},;"#;

        let tests = vec![
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Lparen, "("),
            Token::new(TokenType::Rparen, ")"),
            Token::new(TokenType::Lbrace, "{"),
            Token::new(TokenType::Rbrace, "}"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, "\0"),
        ];

        helper_iter_item(&tests, &mut Lexer::new(input));
    }

    fn expect_items_generator() -> Vec<Token> {
        use TokenType::*;
        vec![
            Token::new(Let, "let"),
            Token::new(Ident, "five"),
            Token::new(Assign, "="),
            Token::new(Int, "5"),
            Token::new(Semicolon, ";"),
            Token::new(Let, "let"),
            Token::new(Ident, "ten"),
            Token::new(Assign, "="),
            Token::new(Int, "10"),
            Token::new(Semicolon, ";"),
            Token::new(Let, "let"),
            Token::new(Ident, "add"),
            Token::new(Assign, "="),
            Token::new(Function, "fn"),
            Token::new(Lparen, "("),
            Token::new(Ident, "x"),
            Token::new(Comma, ","),
            Token::new(Ident, "y"),
            Token::new(Rparen, ")"),
            Token::new(Lbrace, "{"),
            Token::new(Ident, "x"),
            Token::new(Plus, "+"),
            Token::new(Ident, "y"),
            Token::new(Semicolon, ";"),
            Token::new(Rbrace, "}"),
            Token::new(Semicolon, ";"),
            Token::new(Let, "let"),
            Token::new(Ident, "result"),
            Token::new(Assign, "="),
            Token::new(Ident, "add"),
            Token::new(Lparen, "("),
            Token::new(Ident, "five"),
            Token::new(Comma, ","),
            Token::new(Ident, "ten"),
            Token::new(Rparen, ")"),
            Token::new(Semicolon, ";"),
            Token::new(Bang, "!"),
            Token::new(Minus, "-"),
            Token::new(Slash, "/"),
            Token::new(Asterisk, "*"),
            Token::new(Int, "5"),
            Token::new(Semicolon, ";"),
            Token::new(Int, "5"),
            Token::new(Lt, "<"),
            Token::new(Int, "10"),
            Token::new(Gt, ">"),
            Token::new(Int, "5"),
            Token::new(Semicolon, ";"),
            Token::new(If, "if"),
            Token::new(Lparen, "("),
            Token::new(Int, "5"),
            Token::new(Lt, "<"),
            Token::new(Int, "10"),
            Token::new(Rparen, ")"),
            Token::new(Lbrace, "{"),
            Token::new(Return, "return"),
            Token::new(True, "true"),
            Token::new(Semicolon, ";"),
            Token::new(Rbrace, "}"),
            Token::new(Else, "else"),
            Token::new(Lbrace, "{"),
            Token::new(Return, "return"),
            Token::new(False, "false"),
            Token::new(Semicolon, ";"),
            Token::new(Rbrace, "}"),
            Token::new(Int, "10"),
            Token::new(Eq, "=="),
            Token::new(Int, "10"),
            Token::new(Semicolon, ";"),
            Token::new(Int, "10"),
            Token::new(NotEq, "!="),
            Token::new(Int, "9"),
            Token::new(Semicolon, ";"),
            Token::new(Eof, "\0"),
        ]
    }

    #[test]
    fn extend_keyword_support() {
        use super::*;

        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            }else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;

        let tests = expect_items_generator();

        helper_iter_item(&tests, &mut Lexer::new(input));
    }
}
