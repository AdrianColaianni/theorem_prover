/*
 * Theorem prover
 *
 */

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
enum BiOp {
    And,
    Or,
    Xor,
    If,
    Iff,
}

impl fmt::Display for BiOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::And => write!(f, "∧"),
            Self::Or => write!(f, "∨"),
            Self::Xor => write!(f, "⊕"),
            Self::If => write!(f, "→"),
            Self::Iff => write!(f, "↔"),
        }
    }
}

#[allow(dead_code)]
enum Expr {
    Prop(String),
    BiCon(Box<(Expr, BiOp, Expr)>),
    UnCon(Box<(UnOp, Expr)>),
}

impl Expr {
    pub fn get_prepositions(&self) -> u32 {
        match &self {
            Self::Prop(_) => 1,
            Self::BiCon(sub_expr) => {
                (*sub_expr).0.get_prepositions() + (*sub_expr).2.get_prepositions()
            }
            Self::UnCon(sub_expr) => (*sub_expr).1.get_prepositions(),
        }
    }
    fn get_prepositions_vec(&self) -> Vec<&str> {
        let mut vec: Vec<&str> = match &self {
            Self::Prop(prop) => vec![prop],
            Self::BiCon(sub_expr) => {
                let mut vec: Vec<&str> = (*sub_expr).0.get_prepositions_vec();
                let mut vec2: Vec<&str> = (*sub_expr).2.get_prepositions_vec();
                vec.append(&mut vec2);
                vec
            }
            Self::UnCon(sub_expr) => (*sub_expr).1.get_prepositions_vec(),
        };
        vec.sort();
        vec.dedup();
        vec
    }
    fn eval(&self, props: &Dict<bool>) -> bool {
        match &self {
            Self::Prop(prop) => props.get(prop).unwrap().to_owned(),
            Self::BiCon(sub_expr) => {
                let left = (*sub_expr).0.eval(props);
                let right = (*sub_expr).2.eval(props);
                match (*sub_expr).1 {
                    BiOp::And => left && right,
                    BiOp::Or => left || right,
                    BiOp::Xor => (left || right) && !(left && right),
                    BiOp::If => !left || right,
                    BiOp::Iff => left == right,
                }
            }
            Self::UnCon(sub_expr) => match (*sub_expr).0 {
                UnOp::Not => !(*sub_expr).1.eval(props),
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
                        true => "T",
                        false => "F",
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
            Self::BiCon(sub_expr) => {
                write!(f, "({} {} {})", (*sub_expr).0, (*sub_expr).1, (*sub_expr).2,)
            }
            Self::UnCon(sub_expr) => write!(f, "{}{}", (*sub_expr).0, (*sub_expr).1,),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let expr1 = Expr::BiCon(Box::new((
        Expr::Prop("p".to_string()),
        BiOp::Iff,
        Expr::Prop("q".to_string()),
    )));

    println!("{}", expr1);

    expr1.truth_table();

    let expr2 = Expr::BiCon(Box::new((
        Expr::Prop("p".to_string()),
        BiOp::If,
        Expr::Prop("q".to_string()),
    )));

    println!("{}", expr2);

    expr2.truth_table();

    println!("Goodby, world!")
}
