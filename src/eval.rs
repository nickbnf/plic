use lexer;

enum PlicType {
    Integer(usize),
    Operation {
        function: fn(usize, usize) -> usize
    },
    Illegal
}

enum EvalError {
    Close,
    Other
}

pub fn evaluate_line( line: String ) -> Result<usize,()> {
    let mut tokens = lexer::tokens( &line );
    match evaluate_expression( &mut tokens ) {
        Ok(PlicType::Integer(n)) => Ok(n),
        _ => Err(()),
    }
}

fn apply( operation: PlicType, mut operands: Vec<PlicType> ) -> Result<PlicType,EvalError>
{
    match operation {
        PlicType::Operation { function } => {
            let mut s = 0;
            while let Some( PlicType::Integer( n ) ) = operands.pop() {
                s = function( s, n );
            }
            Ok(PlicType::Integer(s))
        },
        _ => Err( EvalError::Other )
    }
}

fn plus( arg1: usize, arg2: usize ) -> usize { arg1 + arg2 }
fn minus( arg1: usize, arg2: usize ) -> usize { arg1 - arg2 }

fn evaluate_expression<T>( mut tokens: &mut T ) -> Result<PlicType,EvalError>
    where T: Iterator<Item=lexer::Token>
{
    match tokens.next() {
        Some(token) => {
            match token {
                lexer::Token::Number(n) => Ok(PlicType::Integer(n)),
                lexer::Token::Plus => Ok(PlicType::Operation { function: plus }),
                lexer::Token::Minus => Ok(PlicType::Operation { function: minus }),
                lexer::Token::ParenOpen => {
                    if let Ok(operation) = evaluate_expression( tokens ) {

                        let mut operands: Vec<PlicType> = vec![];
                        while let Ok( operand ) = evaluate_expression( tokens ) {
                            operands.push( operand );
                        }
                        operands.reverse();
                        // TODO: add error if finishes with anything else than Close

                        apply( operation, operands )
                    }
                    else {
                        Err( EvalError::Other )
                    }
                },
                lexer::Token::ParenClose => Err( EvalError::Close ),
                _ => Err( EvalError::Other ),
            }
        },
        None => Err( EvalError::Other ),
    }
}
