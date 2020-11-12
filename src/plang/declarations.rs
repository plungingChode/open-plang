
use super::variable::*;

pub struct Declarations<'a> {
    variables: Vec<&'a VarDecl>
}