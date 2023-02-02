/*
 * Theorem prover
 *
 */

use core::fmt;

#[allow(dead_code)]
enum UnOperator {
    Not,
}

impl fmt::Display for UnOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Not => write!(f, "¬")
        }
    }
}

#[allow(dead_code)]
enum BiOperator {
    And,
    Or,
    Xor,
    IfThen,
}

impl fmt::Display for BiOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::And => write!(f, "∧"),
            Self::Or => write!(f, "∨"),
            Self::Xor => write!(f, "⊕"),
            Self::IfThen => write!(f, "→")
        }
    }
}

#[allow(dead_code)]
enum Expression {
    Preposition(String),
    BinConditional(Box<(Expression, BiOperator, Expression)>),
    UnConditional(Box<(UnOperator, Expression)>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Preposition(str) => write!(f, "{}", str),
            Self::BinConditional(sub_expr) => write!(
                f,
                "({} {} {})",
                (*sub_expr).0,
                (*sub_expr).1,
                (*sub_expr).2,
            ),
            Self::UnConditional(sub_expr) => write!(
                f,
                "{}{}",
                (*sub_expr).0,
                (*sub_expr).1,
            )
        }
    }
}

fn main() {
    println!("Hello, world!");

    let not_p = Expression::UnConditional(Box::new((
        UnOperator::Not,
        Expression::Preposition(String::from("p")),
    )));

    println!("{:}", not_p);

    let q_or_p = Expression::BinConditional(Box::new((
        Expression::Preposition(String::from("q")),
        BiOperator::Or,
        Expression::Preposition(String::from("p")),
    )));

    println!("{}", q_or_p);

    println!("Goodby, world!")
}
