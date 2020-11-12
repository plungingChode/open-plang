
pub enum UnaryOperator {
    Minus, Sin, Cos, Tan,
    ArcSin, ArcCos, ArcTan,
    Log, Not, Exp, Trunc,
    Round, Real, Upper,
    Lower, IsNum, IsAlpha,
    Pipe
}

impl UnaryOperator {
    pub const fn op(&self) -> &'static str {
        match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Sin => "SIN",
            UnaryOperator::Cos => "COS",
            UnaryOperator::Tan => "TAN",
            UnaryOperator::ArcSin => "ARCSIN",
            UnaryOperator::ArcCos => "ARCCOS",
            UnaryOperator::ArcTan => "ARCTAN",
            UnaryOperator::Log => "LOG",
            UnaryOperator::Not => "NEM",
            UnaryOperator::Exp => "EXP",
            UnaryOperator::Trunc => "EGÉSZ",
            UnaryOperator::Round => "KEREK",
            UnaryOperator::Real => "VALÓS",
            UnaryOperator::Upper => "NAGY",
            UnaryOperator::Lower => "KIS",
            UnaryOperator::IsNum => "SZÁM",
            UnaryOperator::IsAlpha => "BETŰ",
            UnaryOperator::Pipe => "| |"
        }
    }

    pub fn render(&self, e: String /* Expression */) -> String {
        match self {
            UnaryOperator::Pipe => String::from("|") + self.op() + "|",
            _ => String::from(self.op()) + if self.op().len() > 1 { " " } else { "" } + &e
        }
    }
}