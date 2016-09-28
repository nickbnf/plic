use super::PlicType;
use super::EvalError;

pub fn plus( mut operands: Vec<PlicType> ) -> Result<PlicType,EvalError>
{
    let mut s = 0;
    while let Some( PlicType::Integer( n ) ) = operands.pop() {
        s = s + n;
    }
    Ok(PlicType::Integer(s))
}

pub fn minus( operands: Vec<PlicType> ) -> Result<PlicType,EvalError>
{
    Err( EvalError::Other )
}


