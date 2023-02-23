use std::rc::Rc;

use crate::{
    ast::{self, Identifier, Statement},
    lexer::Lexer,
    token::{self, Token, TokenType},
};

struct Parser<'a> {
    lexer: Lexer<'a>,

    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, tok: TokenType) -> bool {
        if self.peek_token.typee == tok {
            self.next_token();
            return true;
        }
        false
    }

    fn cur_token_is(&self, tok: TokenType) -> bool {
        self.current_token.typee == tok
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let tok = self.current_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let identifier = ast::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        //TODO: 跳过对表达式的处理直接遇见分号
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()
        }

        let result = Box::new(ast::LetStatement {
            token: tok,
            name: Rc::from(identifier),
            value: None,
        });
        Some(result)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.typee {
            token::TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parser_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program {
            statements: Vec::new(),
        };

        while self.current_token.typee != token::TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt)
            } else {
                return None;
            }
            self.next_token();
        }
        Some(program)
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::{
        ast::{LetStatement, Node},
        lexer::Lexer,
    };

    use super::Parser;

    #[test]
    fn let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;

        let mut parser = Parser::new(Lexer::new(input));

        let program = parser.parser_program();
        assert!(program.is_some());

        let program = program.unwrap();
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements, got={}",
            program.statements.len()
        );

        let tests = vec![{ "x" }, { "y" }, { "foobar" }];

        for (index, &test) in tests.iter().enumerate() {
            let stmt = &program.statements[index];
            test_let_statement(stmt, test);
        }
    }

    fn test_let_statement(s: &dyn Any, name: &str) {
        let let_stmt = unsafe { s.downcast_ref_unchecked::<Box<LetStatement>>() };

        assert_eq!(
            let_stmt.token_literal(),
            "let",
            "s.token_literal not 'let'. got={}",
            let_stmt.token_literal()
        );

        assert_eq!(
            let_stmt.name.value, name,
            "let_stmt.name.value not '{name}', got={}",
            let_stmt.name.value
        );

        assert_eq!(
            let_stmt.name.token_literal(),
            name,
            "let_stmt.name.token_literal() not '{name}'. got={}",
            let_stmt.name.token_literal()
        );
    }
}
