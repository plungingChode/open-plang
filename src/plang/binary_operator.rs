
pub enum BinaryOperator {
    Plus, Minus, Star, Slash,
    Hat, At, Lt, Gt, Eq, Ne,
    Le, Ge, And, Or, Div, Mod,
    Bracket
}

impl BinaryOperator {
    pub const fn op(&self) -> &'static str {
        match self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Star => "*",
            BinaryOperator::Slash => "/",
            BinaryOperator::Hat => "^",
            BinaryOperator::At => "@",
            BinaryOperator::Lt => "<",
            BinaryOperator::Gt => ">",
            BinaryOperator::Eq => "=",
            BinaryOperator::Ne => "/=",
            BinaryOperator::Le => "<=",
            BinaryOperator::Ge => ">=",
            BinaryOperator::And => "ÉS",
            BinaryOperator::Div => "DIV",
            BinaryOperator::Or => "VAGY",
            BinaryOperator::Mod => "MOD",
            BinaryOperator::Bracket => "[ ]"
        }
    }

    pub fn render(&self, a: String /* Expression */, b: String /* Expression */) -> String {
        match self {
            BinaryOperator::Lt => a + " &lt; " + &b,
            BinaryOperator::Gt => a + " &gt; " + &b,
            BinaryOperator::Le => a + " &lt;= " + &b,
            BinaryOperator::Ge => a + " &gt;= " + &b,
            BinaryOperator::Bracket => a + "[" + &b + "]",
            _ => a + " " + &self.op() + " " + &b
        }
    }
}