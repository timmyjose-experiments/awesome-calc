use crate::ast::{Ast, Op};

pub struct Evaluator;

impl Evaluator {
    pub fn eval(ast: Ast) -> f64 {
        match ast {
            Ast::Number(n) => n,
            Ast::UnaryExpr { op, rhs } => {
                let e = Evaluator::eval(*rhs);
                match op {
                    Op::Add => e,
                    Op::Sub => -e,
                    _ => unreachable!(),
                }
            }
            Ast::BinaryExpr { lhs, op, rhs } => {
                let l = Evaluator::eval(*lhs);
                let r = Evaluator::eval(*rhs);

                match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => l / r,
                }
            }
        }
    }
}
