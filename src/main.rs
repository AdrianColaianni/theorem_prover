/*
 * Theorem prover
 *
 */

use colored::Colorize;
use core::fmt;
use dict::{Dict, DictIface};

#[allow(dead_code)]
enum UnOp {
    Not,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Not => write!(f, "¬"),
        }
    }
}

#[allow(dead_code)]
enum MulOp {
    And,
    Or,
    Xor,
}

impl fmt::Display for MulOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::And => write!(f, "∧"),
            Self::Or => write!(f, "∨"),
            Self::Xor => write!(f, "⊕"),
        }
    }
}

#[allow(dead_code)]
enum BiOp {
    If,
    Iff,
}

impl fmt::Display for BiOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::If => write!(f, "→"),
            Self::Iff => write!(f, "↔"),
        }
    }
}

#[allow(dead_code)]
enum Expr {
    Prop(String),
    UnCon(UnOp, Box<Expr>),
    BiCon(Box<Expr>, BiOp, Box<Expr>),
    MulCon(MulOp, Vec<Expr>),
}

impl Expr {
    pub fn get_prepositions(&self) -> u32 {
        match &self {
            Self::Prop(_) => 1,
            Self::UnCon(_, expr) => (*expr).get_prepositions(),
            Self::BiCon(left, _, right) => (*left).get_prepositions() + (*right).get_prepositions(),
            Self::MulCon(_, exprs) => exprs.into_iter().map(|s| s.get_prepositions()).sum(),
        }
    }
    fn get_prepositions_vec(&self) -> Vec<&str> {
        let mut vec: Vec<&str> = match &self {
            Self::Prop(prop) => vec![prop],
            Self::UnCon(_, expr) => (*expr).get_prepositions_vec(),
            Self::BiCon(left, _, right) => {
                let mut vec: Vec<&str> = (*left).get_prepositions_vec();
                let mut vec2: Vec<&str> = (*right).get_prepositions_vec();
                vec.append(&mut vec2);
                vec
            }
            Self::MulCon(_, exprs) => exprs
                .iter()
                .flat_map(|s| s.get_prepositions_vec())
                .collect(),
        };
        vec.sort();
        vec.dedup();
        vec
    }
    fn eval(&self, props: &Dict<bool>) -> bool {
        match &self {
            Self::Prop(prop) => props.get(prop).unwrap().to_owned(),
            Self::UnCon(op, expr) => match op {
                UnOp::Not => !(*expr).eval(props),
            },
            Self::BiCon(left, op, right) => {
                let left = (*left).eval(props);
                let right = (*right).eval(props);
                match op {
                    BiOp::If => !left || right,
                    BiOp::Iff => left == right,
                }
            }
            Self::MulCon(op, exprs) => match op {
                MulOp::And => exprs.iter().all(|s| s.eval(&props)),
                MulOp::Or => exprs.iter().any(|s| s.eval(&props)),
                MulOp::Xor => {
                    exprs
                        .iter()
                        .map(|s| match s.eval(&props) {
                            true => 1,
                            false => 0,
                        })
                        .sum::<i32>()
                        == 1
                }
            },
        }
    }
    pub fn truth_table(&self) {
        let props: Vec<&str> = self.get_prepositions_vec();
        let rows = 2_usize.pow(props.len() as u32);

        for prop in &props {
            print!("{} | ", prop);
        }

        let mut truth_vec: Vec<Vec<bool>> = vec![vec![false; props.len() + 1]; rows as usize];

        println!();
        for index in 0..rows {
            for prop_index in 0..props.len() {
                let magic = rows / (1 + prop_index);
                truth_vec[index][prop_index] = index % magic < magic / 2;
            }
        }
        for index in 0..rows {
            let mut dict = Dict::<bool>::new();
            for (pi, prop) in props.iter().enumerate() {
                dict.add(prop.to_owned().to_owned(), truth_vec[index][pi]);
            }
            truth_vec[index][props.len()] = self.eval(&dict);
        }
        // println!("{:?}", truth_vec);
        for row in truth_vec {
            for item in row {
                print!(
                    "{} | ",
                    match item {
                        true => format!("T").green(),
                        false => format!("F").red(),
                    }
                );
            }
            println!();
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Prop(str) => write!(f, "{}", str),
            Self::UnCon(op, expr) => write!(f, "{}{}", op, (*expr)),
            Self::BiCon(left, op, right) => {
                write!(f, "({} {} {})", (*left), op, (*right))
            }
            Self::MulCon(op, exprs) => write!(
                f,
                "({})",
                exprs
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join(&format!(" {} ", op)),
            ),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let expr1 = Expr::MulCon(
        MulOp::And,
        vec![
            Expr::Prop("a".to_string()),
            Expr::BiCon(
                Box::new(Expr::UnCon(
                    UnOp::Not,
                    Box::new(Expr::Prop("a".to_string())),
                )),
                BiOp::If,
                Box::new(Expr::Prop("b".to_string())),
                ),
            Expr::Prop("c".to_string())
        ],
    );

    println!("{}", expr1);

    expr1.truth_table();

    let expr2 = Expr::MulCon(
        MulOp::And,
        vec![
            Expr::Prop("a".to_string()),
            Expr::Prop("b".to_string()),
            Expr::Prop("c".to_string()),
            Expr::Prop("d".to_string()),
            Expr::Prop("e".to_string()),
            // Expr::Prop("f".to_string()),
        ]
    );

    println!("{}", expr2);

    expr2.truth_table();

    println!("Goodby, world!")
}
