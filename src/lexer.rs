// Our lexer

use std;

pub fn tokens<'a>( code: &'a str ) -> CodeTokens<'a> {
    CodeTokens { iter: code.chars() }
}

#[derive(Clone, Debug)]
pub enum Token {
    ParenOpen,
    ParenClose,
    Plus,
    Minus,
    Multiply,
    Divide,
    Illegal,
    Number( usize )
}

pub struct CodeTokens<'a> {
    iter: std::str::Chars<'a>,
}

impl<'a> Iterator for CodeTokens<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let c = self.iter.next();
        println!( "Token: {:?}", c );
        match c {
            Some(car) => match car {
                '(' => Some(Token::ParenOpen),
                ')' => Some(Token::ParenClose),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Multiply),
                '/' => Some(Token::Divide),
                n @ '0' ... '9' => {
                    let res = usize::from_str_radix(&n.to_string(), 10);
                    match res {
                        Ok( number ) => Some(Token::Number( number )),
                        _ => panic!()
                    }
                },
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

    #[test]
    fn recognise_numbers() {
        let string = "1239";
        let mut tokens = super::tokens( string );
        assert!( matches!( tokens.next(), Some(super::Token::Number(1)) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Number(2)) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Number(3)) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Number(9)) ) );
        assert!( matches!( tokens.next(), None ) );
    }
}
