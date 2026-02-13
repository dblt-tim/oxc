
use crate::ast::{
    types::*,
    statement::*,
};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct FunctionArg {
    name: String,
    ty: Type,
}
#[allow(unused)]
impl FunctionArg {
    pub fn new(name: String, ty: Type) -> Self {
        Self {
            name,
            ty,
        }
    }
    pub fn get_name(&mut self) -> &mut String {
        return &mut self.name;
    }
    pub fn get_type(&mut self) -> &mut Type {
        return &mut self.ty;
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct FunctionProto {
    name: String,
    args: Vec<FunctionArg>,
    ret_ty: Type,
}
#[allow(unused)]
impl FunctionProto {
    pub fn new(name: String, args: Vec<FunctionArg>, ret_ty: Type) -> Self {
        Self {
            name,
            args,
            ret_ty,
        }
    }
    pub fn get_name(&mut self) -> &mut String {
        return &mut self.name;
    }
    pub fn get_args(&mut self) -> &mut Vec<FunctionArg> {
        return &mut self.args;
    }
    pub fn get_ret_ty(&mut self) -> &mut Type {
        return &mut self.ret_ty;
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Function {
    proto: FunctionProto,
    body: Vec<Statement>,
}
#[allow(unused)]
impl Function {
    pub fn new(proto: FunctionProto, body: Vec<Statement>) -> Self {
        Self {
            proto,
            body,
        }
    }
    pub fn get_proto(&mut self) -> &mut FunctionProto {
        return &mut self.proto;
    }
    pub fn get_body(&mut self) -> &mut Vec<Statement> {
        return &mut self.body;
    }
}