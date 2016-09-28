/// Implements our evaluator!

mod builtins;

use lexer;

/// A fully-reduced expression
pub enum PlicType {
    Integer(usize),
    Operation {
        function: fn( mut operands: Vec<PlicType> ) -> Result<PlicType,EvalError>
    },
    Illegal
}

/// An error returned by the evaluator
pub enum EvalError {
    /// The evaluator has encountered a closing parenthesis, not an error per se.
    Close,
    /// The operation in a combination cannot be applied.
    NonApplicable,
    /// Other error.
    Other
}

/// Parse and evaluate the passed string
pub fn evaluate_line( line: String ) -> Result<usize,()> {
    let mut tokens = lexer::tokens( &line );
    match evaluate_expression( &mut tokens ) {
        Ok(PlicType::Integer(n)) => Ok(n),
        _ => Err(()),
    }
}

fn apply( operation: PlicType, operands: Vec<PlicType> ) -> Result<PlicType,EvalError>
{
    match operation {
        PlicType::Operation { function } => {
            function( operands )
        },
        _ => Err( EvalError::NonApplicable )
    }
}

fn evaluate_expression<T>( mut tokens: &mut T ) -> Result<PlicType,EvalError>
    where T: Iterator<Item=lexer::Token>
{
    if let Some(token) = tokens.next() {
        match token {
            lexer::Token::Number(n) => Ok(PlicType::Integer(n)),
            lexer::Token::Plus => Ok(PlicType::Operation { function: builtins::plus }),
            lexer::Token::Minus => Ok(PlicType::Operation { function: builtins::minus }),
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
    }
    else {
        Err( EvalError::Other )
    }
}

#[cfg(test)]
mod tests {
    use lexer;

    #[test]
    fn one_builtin() {
        let tokens = vec![
            lexer::Token::ParenOpen,
            lexer::Token::Plus,
            lexer::Token::Number(1),
            lexer::Token::Number(2),
            lexer::Token::ParenClose,
        ];

        assert!(
            matches!( super::evaluate_expression( &mut tokens.into_iter() ),
            Ok( super::PlicType::Integer(3) ) )
        );
    }

    #[test]
    fn nested_builtins() {
        let tokens = vec![
            lexer::Token::ParenOpen,
            lexer::Token::Plus,
            lexer::Token::ParenOpen,
            lexer::Token::Plus,
            lexer::Token::ParenOpen,
            lexer::Token::Plus,
            lexer::Token::Number(1),
            lexer::Token::Number(2),
            lexer::Token::ParenClose,
            lexer::Token::ParenClose,
            lexer::Token::ParenOpen,
            lexer::Token::Plus,
            lexer::Token::Number(10),
            lexer::Token::Number(20),
            lexer::Token::ParenClose,
            lexer::Token::ParenClose,
        ];

        assert!(
            matches!( super::evaluate_expression( &mut tokens.into_iter() ),
            Ok( super::PlicType::Integer(33) ) )
        );
    }

    #[test]
    fn error_non_applicable() {
        let tokens = vec![
            lexer::Token::ParenOpen,
            lexer::Token::Number(1),
            lexer::Token::Number(2),
            lexer::Token::Number(3),
            lexer::Token::ParenClose,
        ];

        assert!(
            matches!( super::evaluate_expression( &mut tokens.into_iter() ),
            Err( super::EvalError::NonApplicable ) )
        );
    }
}
