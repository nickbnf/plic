// Our lexer

use std;

pub fn tokens<'a>( code: &'a str ) -> CodeTokens<'a> {
    CodeTokens { iter: code.chars().peekable() }
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
    Number( usize ),
    Word( String ),
}

pub struct CodeTokens<'a> {
    iter: std::iter::Peekable<std::str::Chars<'a>>,
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
                    let mut s = String::new();
                    s.push( n );
                    while let Some(&'0' ... '9') = self.iter.peek() {
                        s.push( self.iter.next().unwrap() );
                    }
                    let res = usize::from_str_radix(&s, 10);
                    match res {
                        Ok( number ) => Some(Token::Number( number )),
                        _ => panic!()
                    }
                },
                c @ 'A' ... 'Z' | c @ 'a' ... 'z' | c @ '_' => {
                    let mut s = String::new();
                    s.push( c );
                    loop {
                        match self.iter.peek() {
                            Some(&'A' ... 'Z') | Some(&'a' ... 'z') | Some(&'_') =>
                                s.push( self.iter.next().unwrap() ),
                            _ => break,
                        }
                    }
                    Some(Token::Word( s ))
                },

                ' ' | '\t' | '\n' => self.next(),
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
        assert!( matches!( tokens.next(), Some(super::Token::Number(1239)) ) );
        assert!( matches!( tokens.next(), None ) );
    }

    #[test]
    fn recognise_one_char_followed_by_numbers() {
        let string = "(19 76";
        let mut tokens = super::tokens( string );
        assert!( matches!( tokens.next(), Some(super::Token::ParenOpen) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Number(19)) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Number(76)) ) );
        assert!( matches!( tokens.next(), None ) );
    }

    #[test]
    fn recognise_various_blanks() {
        let string = " ( +  19\t76   \n)";
        let mut tokens = super::tokens( string );
        assert!( matches!( tokens.next(), Some(super::Token::ParenOpen) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Plus) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Number(19)) ) );
        assert!( matches!( tokens.next(), Some(super::Token::Number(76)) ) );
        assert!( matches!( tokens.next(), Some(super::Token::ParenClose) ) );
        assert!( matches!( tokens.next(), None ) );
    }

    #[test]
    fn recognise_two_words() {
        let string = "(word_one Word_TWO )";
        let mut tokens = super::tokens( string );
        assert!( matches!( tokens.next(), Some(super::Token::ParenOpen) ) );
        assert!( match tokens.next() {
            Some(super::Token::Word(s)) => s == "word_one",
            _ => false } );
        assert!( match tokens.next() {
            Some(super::Token::Word(s)) => s == "Word_TWO",
            _ => false } );
        assert!( matches!( tokens.next(), Some(super::Token::ParenClose) ) );
        assert!( matches!( tokens.next(), None ) );
    }
}
