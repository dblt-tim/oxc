
mod ast;
use ast::{
    functions::{Function, FunctionProto},
    types::Type,
    expression::Expression,
    statement::Statement,
};

fn main() {
    
    let hardcoded_ast = Function::new(
        FunctionProto::new(String::from("main"), vec![], Type::I32),
        vec![
            Statement::Return(Expression::ConstantInteger(42)),
        ],
    );
}