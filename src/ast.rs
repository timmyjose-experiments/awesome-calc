use std::fmt;

#[derive(Debug)]
pub enum Ast {
    Number(f64),
    UnaryExpr {
        op: Op,
        rhs: Box<Ast>,
    },
    BinaryExpr {
        lhs: Box<Ast>,
        op: Op,
        rhs: Box<Ast>,
    },
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Ast::Number(ref n) => n.to_string(),
                Ast::UnaryExpr { ref op, ref rhs } => {
                    format!("{op}{rhs}")
                }
                Ast::BinaryExpr {
                    ref lhs,
                    ref op,
                    ref rhs,
                } => {
                    format!("{lhs}{op}{rhs}")
                }
            }
        )
    }
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
            }
        )
    }
}
