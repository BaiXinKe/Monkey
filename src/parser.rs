use crate::{ast, lexer::Lexer, token::Token};

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,

    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Self {
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

    fn parser_program(&self) -> Option<ast::Program> {
        None
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

        let mut lexer = Lexer::new(input);
        let parser = Parser::new(&mut lexer);

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
        let let_stmt = s.downcast_ref::<LetStatement>();
        assert!(let_stmt.is_some());
        let let_stmt = let_stmt.unwrap();

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
