
use crate::ast::expression::Expression;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
    UNKNOWN,
}

