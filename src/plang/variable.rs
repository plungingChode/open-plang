use super::lexer::*;
use super::unary_operator::*;
use super::binary_operator::*;

pub enum Error {
    None,
    Name,
    Colon,
    Type,
    DupName
}

pub trait Type {
    fn const_sval(&self) -> String;
    fn const_nval(&self) -> f64;
    fn render(&self) -> String;
    fn can_copy(&self, var: Box<dyn Type>) -> bool;
    fn copy(&self, var: Box<dyn Type>) -> Box<dyn Type>;
    fn init_val(&self) -> Box<dyn Type>;
    
    fn unary_optype(&self, op: UnaryOperator) -> Box<dyn Type>;
    fn unary_apply(&self, var: Box<dyn Type>) -> Box<dyn Type>;

    // I dunno about these
    fn binary_optype(&self, op: BinaryOperator, var: Box<dyn Type>) -> Box<dyn Type>;
    fn binary_apply(&self, op: BinaryOperator, t: Box<dyn Type>, var1: Box<dyn Type>, var2: Box<dyn Type>) -> Box<dyn Type>;
    fn binary_has_accessor(&self, op: BinaryOperator, x: Box<dyn Type>) -> bool;
    fn access(&self, op: BinaryOperator, a: Box<dyn Type>, b: Box<dyn Type>, c: Box<dyn Type>) -> Box<dyn Type>;

    fn print_data(&self, s: &mut String) { s.push_str(&self.render())}
}

impl Type {
    const fn null() -> NullType {
        NullType
    } 
}

pub struct NullType;

impl Type for NullType {
    fn const_sval(&self) -> String { unimplemented!() }
    fn const_nval(&self) -> f64 { unimplemented!() }

    fn render(&self) -> String { unimplemented!() }
    fn can_copy(&self, var: Box<dyn Type>) -> bool { unimplemented!() }
    fn copy(&self, var: Box<dyn Type>) -> Box<dyn Type> { unimplemented!() }
    fn init_val(&self) -> Box<dyn Type> { unimplemented!() }
    
    fn unary_optype(&self, op: UnaryOperator) -> Box<dyn Type> { unimplemented!() }
    fn unary_apply(&self, var: Box<dyn Type>) -> Box<dyn Type> { unimplemented!() }

    // I dunno about these
    fn binary_optype(&self, op: BinaryOperator, var: Box<dyn Type>) -> Box<dyn Type> { unimplemented!() }
    fn binary_apply(&self, op: BinaryOperator, t: Box<dyn Type>, var1: Box<dyn Type>, var2: Box<dyn Type>) -> Box<dyn Type> { unimplemented!() }
    fn binary_has_accessor(&self, op: BinaryOperator, x: Box<dyn Type>) -> bool { unimplemented!() }
    fn access(&self, op: BinaryOperator, a: Box<dyn Type>, b: Box<dyn Type>, c: Box<dyn Type>) -> Box<dyn Type> { unimplemented!() }
}

pub struct VarDecl {
    names: Vec<String>,
    is_last: bool,
    err: Error,
    err_idx: i32,
    vtype: Box<dyn Type>,
}

impl VarDecl {
    fn new(names: Vec<String>, vtype: Box<dyn Type>, err: Error) -> VarDecl {
        VarDecl { names, vtype, is_last: false, err, err_idx: -1 }
    }

    fn parse(lex: &mut Lexer) -> VarDecl {
        let mut names: Vec<String> = vec![];

        if !lex.is_ident() {
            return VarDecl::new(names, Box::from(Type::null()), Error::Name);
        }

        names.push(lex.lexical());
        lex.next();
        while lex.sval() == "," {
            lex.next();
            if !lex.is_ident() {
                return VarDecl::new(names, Box::from(Type::null()), Error::Name);
            }
            names.push(lex.lexical());
            lex.next();
        }

        if lex.sval() != ":" {
            return VarDecl::new(names, Box::from(Type::null()), Error::Colon);
        }
        lex.next();

        // TODO parse type

        return VarDecl::new(vec![], Box::from(Type::null()), Error::None);
    }
}