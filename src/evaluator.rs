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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test1() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("3"));
        let expected = 3.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test2() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-7"));
        let expected = -7.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test3() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("+42"));
        let expected = 42.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test4() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("1 + 2"));
        let expected = 3.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test5() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("10 - 5"));
        let expected = 5.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test6() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("5 + -3"));
        let expected = 2.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test7() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("5 - -3"));
        let expected = 8.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test8() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("2 * 3"));
        let expected = 6.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test9() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("8 / 2"));
        let expected = 4.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test10() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-6 * 4"));
        let expected = -24.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test11() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-8 / -4"));
        let expected = 2.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test12() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("2 + 3 * 4"));
        let expected = 14.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test13() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("10 - 2 * 5"));
        let expected = 0.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test14() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("8 / 4 + 2"));
        let expected = 4.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test15() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("6 + 4 / 2"));
        let expected = 8.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test16() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("5 * 2 - 3"));
        let expected = 7.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test17() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("(2 + 3) * 4 "));
        let expected = 20.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test18() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("10 / (5 - 3)"));
        let expected = 5.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test19() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("(8 + 2) * (3 - 1)"));
        let expected = 20.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test20() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("(6 - 2) * (3 + 1)"));
        let expected = 16.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test21() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("((2 + 3) * (4 - 1)) + 5 "));
        let expected = 20.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test22() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("10 - (3 * (2 + 1)) "));
        let expected = 1.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test23() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("(4 / (1 + 1)) * (3 + 5)"));
        let expected = 16.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test24() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("3 * (2 + (4 - 1)) "));
        let expected = 15.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test25() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-(-3 + 5) "));
        let expected = -2.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test26() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-(3 + -2) "));
        let expected = -1.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test27() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("+(3 + -4)"));
        let expected = -1.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test28() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-((2 + 3) * 4)"));
        let expected = -20.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test29() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("+5"));
        let expected = 5.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test30() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-5"));
        let expected = -5.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }
    #[test]
    fn test31() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("-(-5)"));
        let expected = 5.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test32() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("+(-5)"));
        let expected = -5.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test33() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("3 + (4 * 2) / (1 - 5)"));
        let expected = 1.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test34() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("(8 - 3) * (-2) "));
        let expected = -10.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test35() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("(10 - (3 * 2 + 1))"));
        let expected = 3.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test36() -> eyre::Result<()> {
        let mut parser = Parser::new(Lexer::new("6 + (8 - (2 * 3)) * 4 "));
        let expected = 14.0;
        let actual = Evaluator::eval(parser.parse()?);

        assert_eq!(expected, actual);

        Ok(())
    }
}
