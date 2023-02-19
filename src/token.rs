// token

use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenType {
    Illegal,
    Eof,

    // 标识符 + 字面量
    Ident,
    Int,

    // 运算符
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Eq,
    NotEq,

    // 分隔符
    Comma,
    Semicolon,

    // 括号
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // 关键字
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        match self {
            Illegal => write!(f, "ILLEGAL"),
            Eof => write!(f, "EOF"),
            Ident => write!(f, "IDENT"),
            Int => write!(f, "INT"),
            Assign => write!(f, "="),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Bang => write!(f, "!"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Eq => write!(f, "=="),
            NotEq => write!(f, "!="),
            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),
            Lparen => write!(f, "("),
            Rparen => write!(f, ")"),
            Lbrace => write!(f, "{{"),
            Rbrace => write!(f, "}}"),
            Function => write!(f, "fn"),
            Let => write!(f, "let"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Return => write!(f, "return"),
        }
    }
}

// ------------------------------- Token --------------------------------------------------
#[derive(Debug)]
pub struct Token {
    pub(crate) typee: TokenType,
    pub(crate) literal: String,
}

impl Token {
    pub(crate) fn new(ty: TokenType, literal: &str) -> Self {
        Self {
            typee: ty,
            literal: literal.to_owned(),
        }
    }

    pub(crate) fn lookup_ident(ident: &str) -> TokenType {
        static KEY_WORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
            let mut key_words = HashMap::new();
            key_words.insert("fn".to_owned(), TokenType::Function);
            key_words.insert("let".to_owned(), TokenType::Let);
            key_words.insert("true".to_owned(), TokenType::True);
            key_words.insert("false".to_owned(), TokenType::False);
            key_words.insert("if".to_owned(), TokenType::If);
            key_words.insert("else".to_owned(), TokenType::Else);
            key_words.insert("return".to_owned(), TokenType::Return);
            key_words
        });

        if let Some(token_type) = KEY_WORDS.get(ident) {
            *token_type
        } else {
            TokenType::Ident
        }
    }
}
