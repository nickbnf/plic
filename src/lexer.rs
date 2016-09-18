// Our lexer

use std;

pub fn tokens<'a>( code: &'a str ) -> CodeTokens<'a> {
    CodeTokens { iter: code.chars() }
}

#[derive(Debug)]
pub enum Token {
    ParenOpen,
    ParenClose,
    Plus,
    Minus,
    Multiply,
    Divide,
    Illegal,
}

pub struct CodeTokens<'a> {
    iter: std::str::Chars<'a>,
}

impl<'a> Iterator for CodeTokens<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let c = self.iter.next();
        match c {
            Some(car) => match car {
                '(' => Some(Token::ParenOpen),
                ')' => Some(Token::ParenClose),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Multiply),
                '/' => Some(Token::Divide),
                _ => Some(Token::Illegal)
            },
            None      => None
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn one_char_tokens() {
        let string = "()+-*/";
        let mut tokens = super::tokens( string );
        assert!( matches!( tokens.next(), Some(super::Token::ParenOpen) ) );
        assert!( matches!( tokens.next(), Some(super::Token::ParenClose) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Plus) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Minus) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Multiply) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Divide) ) );
        assert!( matches!( tokens.next(), None ) );
    }
}
