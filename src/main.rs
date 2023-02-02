/*
 * Theorem prover
 *
 */

use core::fmt;
use dict::{Dict, DictIface};

#[allow(dead_code)]
enum UnOperator {
    Not,
}

impl fmt::Display for UnOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Not => write!(f, "¬"),
        }
    }
}

#[allow(dead_code)]
enum BiOperator {
    And,
    Or,
    Xor,
}

impl fmt::Display for BiOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::And => write!(f, "∧"),
            Self::Or => write!(f, "∨"),
            Self::Xor => write!(f, "⊕"),
        }
    }
}

#[allow(dead_code)]
enum Expression {
    Proposition(String),
    BinConditional(Box<(Expression, BiOperator, Expression)>),
    UnConditional(Box<(UnOperator, Expression)>),
}

impl Expression {
    pub fn get_prepositions(&self) -> u32 {
        match &self {
            Self::Proposition(_) => 1,
            Self::BinConditional(sub_expr) => {
                (*sub_expr).0.get_prepositions() + (*sub_expr).2.get_prepositions()
            }
            Self::UnConditional(sub_expr) => (*sub_expr).1.get_prepositions(),
        }
    }
    fn get_prepositions_vec(&self) -> Vec<&str> {
        let mut vec: Vec<&str> = match &self {
            Self::Proposition(prop) => vec![prop],
            Self::BinConditional(sub_expr) => {
                let mut vec: Vec<&str> = (*sub_expr).0.get_prepositions_vec();
                let mut vec2: Vec<&str> = (*sub_expr).2.get_prepositions_vec();
                vec.append(&mut vec2);
                vec
            }
            Self::UnConditional(sub_expr) => (*sub_expr).1.get_prepositions_vec(),
        };
        vec.sort();
        vec.dedup();
        vec
    }
    fn eval(&self, props: &Dict<bool>) -> bool {
        match &self {
            Self::Proposition(prop) => props.get(prop).unwrap().to_owned(),
            Self::BinConditional(sub_expr) => match (*sub_expr).1 {
                BiOperator::And => (*sub_expr).0.eval(props) && (*sub_expr).2.eval(props),
                BiOperator::Or => (*sub_expr).0.eval(props) || (*sub_expr).2.eval(props),
                BiOperator::Xor => {
                    ((*sub_expr).0.eval(props) || (*sub_expr).2.eval(props))
                        && !((*sub_expr).0.eval(props) && (*sub_expr).2.eval(props))
                }
            },
            Self::UnConditional(sub_expr) => match (*sub_expr).0 {
                UnOperator::Not => !(*sub_expr).1.eval(props),
            },
        }
    }
    pub fn truth_table(&self) {
        let props: Vec<&str> = self.get_prepositions_vec();
        let rows = 2_usize.pow(props.len() as u32);

        for prop in &props {
            print!("{} | ", prop);
        }

        let mut truth_vec: Vec<Vec<bool>> = vec![vec![false; props.len()+1]; rows as usize];

        println!();
        for index in 0..rows {
            for prop_index in 0..props.len() {
                let magic = rows/(1+prop_index);
                truth_vec[index][prop_index] = index % magic < magic/2;
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
                print!("{} | ", match item {
                    true => "T",
                    false => "F",
                });
            }
            println!();
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Proposition(str) => write!(f, "{}", str),
            Self::BinConditional(sub_expr) => {
                write!(f, "({} {} {})", (*sub_expr).0, (*sub_expr).1, (*sub_expr).2,)
            }
            Self::UnConditional(sub_expr) => write!(f, "{}{}", (*sub_expr).0, (*sub_expr).1,),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let not_p = Expression::UnConditional(Box::new((
        UnOperator::Not,
        Expression::Proposition(String::from("p")),
    )));

    println!("{}", not_p);

    not_p.truth_table();

    let q_or_p = Expression::BinConditional(Box::new((
        Expression::Proposition(String::from("q")),
        BiOperator::Or,
        Expression::Proposition(String::from("p")),
    )));

    println!("{}", q_or_p);

    q_or_p.truth_table();

    let q_and_p_or_r = Expression::BinConditional(Box::new((
        Expression::BinConditional(Box::new((
        Expression::Proposition(String::from("q")),
        BiOperator::And,
        Expression::Proposition(String::from("p")),
        ))),
        BiOperator::Or,
        Expression::Proposition(String::from("r")),
    )));

    println!("{}", q_and_p_or_r);

    q_and_p_or_r.truth_table();

    println!("Goodby, world!")
}
