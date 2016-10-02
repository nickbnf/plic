/// Implements our evaluator!

mod builtins;

use lexer;

use std::fmt;
use std::collections::HashMap;

type HeapIndex = usize;

/// A plic expression
#[derive(Clone)]
pub enum PlicType {
    /// Integer (atom)
    Integer(usize),
    /// Primitive (built-in) operation (atom)
    Operation {
        function: fn( operands: Vec<PlicType> ) -> Result<PlicType,EvalError>
    },
    /// A symbol (atom)
    Symbol(String),
    /// A pair, maybe used as the head of a list
    Pair(HeapIndex, HeapIndex)
}

impl fmt::Display for PlicType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PlicType::Integer( n ) => write!(f, "{}", n),
            &PlicType::Operation { function: _ } => write!(f, "Built-in operation"),
            &PlicType::Symbol( ref s ) => write!(f, "'{}", s),
            _ => write!(f, "Unknown")
        }
    }
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

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &EvalError::NonApplicable => write!(f, "Operation not applicable"),
            &EvalError::Other => write!(f, "Error"),
            _ => write!(f, "Unknown")
        }
    }
}

/// An environment, a list of binding to their values
///
/// Environments are stacked with the head of the list being the insidemost frame.
/// The outsidemost is the global environment.
struct Environment {
    bindings: HashMap<String, PlicType>
}

impl Environment {
    fn new() -> Environment {
        Environment { bindings: HashMap::new() }
    }
}

/// Parse and evaluate the passed string
pub fn evaluate_line( line: String ) -> Result<String,String> {
    let mut global_env = Environment::new();
    let mut tokens = lexer::tokens( &line );
    match evaluate_expression( &mut tokens, &mut global_env ) {
        Ok( r ) => Ok( format!( "{}", r ) ),
        Err( e ) => Err( format!( "{}", e ) ),
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

fn evaluate_expression<T>( mut tokens: &mut T, env: &mut Environment ) -> Result<PlicType,EvalError>
    where T: Iterator<Item=lexer::Token>
{
    if let Some(token) = tokens.next() {
        match token {
            lexer::Token::Number(n) => Ok(PlicType::Integer(n)),
            lexer::Token::Plus => Ok(PlicType::Operation { function: builtins::plus }),
            lexer::Token::Minus => Ok(PlicType::Operation { function: builtins::minus }),
            lexer::Token::Word(w) => match w {
                _ => match env.bindings.get(&w) {
                    Some(ref value) => { let copy = (*value).clone(); Ok(copy) },
                    None => Ok(PlicType::Symbol(w))
                }
            },
            lexer::Token::ParenOpen => {
                if let Ok(operation) = evaluate_expression( tokens, env ) {
                    let mut operands: Vec<PlicType> = vec![];
                    while let Ok( operand ) = evaluate_expression( tokens, env ) {
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
        let mut env = super::Environment::new();
        let tokens = vec![
            lexer::Token::ParenOpen,
            lexer::Token::Plus,
            lexer::Token::Number(1),
            lexer::Token::Number(2),
            lexer::Token::ParenClose,
        ];

        assert!(
            matches!( super::evaluate_expression( &mut tokens.into_iter(), &mut env ),
            Ok( super::PlicType::Integer(3) ) )
        );
    }

    #[test]
    fn nested_builtins() {
        let mut env = super::Environment::new();
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
            matches!( super::evaluate_expression( &mut tokens.into_iter(), &mut env ),
            Ok( super::PlicType::Integer(33) ) )
        );
    }

    #[test]
    fn error_non_applicable() {
        let mut env = super::Environment::new();
        let tokens = vec![
            lexer::Token::ParenOpen,
            lexer::Token::Number(1),
            lexer::Token::Number(2),
            lexer::Token::Number(3),
            lexer::Token::ParenClose,
        ];

        assert!(
            matches!( super::evaluate_expression( &mut tokens.into_iter(), &mut env ),
            Err( super::EvalError::NonApplicable ) )
        );
    }
}
