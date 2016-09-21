#[macro_use] extern crate matches;

extern crate rustyline;

mod lexer;

fn main() {
    repl();
}

fn repl() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        let result = match readline {
            Ok(line) => evaluate_line( line ),
            Err(_)   => return(),
        };
        match result {
            Ok(n) => println!("Result: {}", n),
            _ => println!("Error"),
        };
    }
}

fn evaluate_line( line: String ) -> Result<usize,()> {
    let mut tokens = lexer::tokens( &line );
    match evaluate_expression( &mut tokens ) {
        Ok(PlicType::Integer(n)) => Ok(n),
        _ => Err(()),
    }
}

enum PlicType {
    Integer(usize),
    Operation {
        arity: usize,
        function: Box<Fn(usize, usize) -> usize>
    },
    Illegal
}

fn evaluate_expression( mut tokens: &mut lexer::CodeTokens ) -> Result<PlicType,()>
{
    match tokens.next() {
        Some(token) => {
            match token {
                lexer::Token::Number(n) => Ok(PlicType::Integer(n)),
                lexer::Token::Plus => Ok(PlicType::Operation {
                    arity: 2,
                    function: Box::new(|arg1, arg2| arg1+arg2) }),
                lexer::Token::Minus => Ok(PlicType::Operation {
                    arity: 2,
                    function: Box::new(|arg1, arg2| arg1-arg2) }),
                lexer::Token::ParenOpen => {
                    let operation = evaluate_expression( &mut tokens );
                    match operation {
                        Ok(PlicType::Operation { arity, function } ) => {
                            let operand1 = evaluate_expression( &mut tokens );
                            let operand2 = evaluate_expression( &mut tokens );
                            let closing = tokens.next();
                            match (operand1, operand2, closing) {
                                ( Ok(PlicType::Integer(n1)),
                                  Ok(PlicType::Integer(n2)),
                                  Some( lexer::Token::ParenClose ) ) =>
                                    Ok(PlicType::Integer(function(n1, n2))),
                                _ => Err(()),
                            }
                        },
                        _ => Err(())
                    }
                },
                _ => Err(()),
            }
        },
        None => Err(()),
    }
}
