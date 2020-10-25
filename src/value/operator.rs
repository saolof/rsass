use crate::css::Value;
use crate::value::{ListSeparator, Quotes, Unit};
use crate::Rational;
use num_traits::Zero;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    And,
    Or,
    Equal,
    NotEqual,
    Greater,
    GreaterE,
    Lesser,
    LesserE,

    Plus,
    Minus,
    Multiply,
    Div,
    Modulo,

    Not,
}

impl Operator {
    pub fn eval(&self, a: Value, b: Value) -> Option<Value> {
        match *self {
            Operator::And => Some(Value::bool(a.is_true() && b.is_true())),
            Operator::Or => {
                if a.is_true() {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            Operator::Equal => Some(Value::bool(a == b)), // equal_values(&a, &b))),
            Operator::NotEqual => Some(Value::bool(a != b)), // !equal_values(&a, &b))),
            Operator::Greater => Some(Value::bool(a > b)),
            Operator::GreaterE => Some(Value::bool(a >= b)),
            Operator::Lesser => Some(Value::bool(a < b)),
            Operator::LesserE => Some(Value::bool(a <= b)),
            Operator::Plus => match (a, b) {
                (Value::Color(a, _), Value::Numeric(bn, Unit::None, _)) => {
                    let bn = bn.value;
                    Some(Value::Color(a + bn, None))
                }
                (Value::Color(a, _), Value::Color(b, _)) => {
                    Some(Value::Color(a + b, None))
                }
                (Value::Numeric(a, au, ..), Value::Numeric(b, bu, ..)) => {
                    if au == bu || bu == Unit::None {
                        Some(Value::Numeric(a + b, au, true))
                    } else if au == Unit::None {
                        Some(Value::Numeric(a + b, bu, true))
                    } else if let Some(scale) = bu.scale_to(&au) {
                        Some(Value::Numeric(
                            (a.value + b.value * scale).into(),
                            au,
                            true,
                        ))
                    } else {
                        None
                    }
                }
                (Value::Literal(a, Quotes::None), Value::Literal(b, _)) => {
                    Some(Value::Literal(format!("{}{}", a, b), Quotes::None))
                }
                (Value::Literal(a, _), Value::Literal(b, _)) => Some(
                    Value::Literal(format!("{}{}", a, b), Quotes::Double),
                ),
                (Value::Literal(a, q), b) => Some(Value::Literal(
                    format!("{}{}", a, b.format(Default::default())),
                    q,
                )),
                (a, Value::Literal(b, q)) => Some(Value::Literal(
                    format!("{}{}", a.format(Default::default()), b),
                    q,
                )),
                _ => None,
            },
            Operator::Minus => match (&a, &b) {
                (
                    &Value::Color(ref a, _),
                    &Value::Numeric(ref bn, Unit::None, _),
                ) => {
                    let bn = bn.value;
                    Some(Value::Color(a - bn, None))
                }
                (&Value::Color(ref a, _), &Value::Color(ref b, _)) => {
                    Some(Value::Color(a - b, None))
                }
                (
                    &Value::Numeric(ref av, ref au, ..),
                    &Value::Numeric(ref bv, ref bu, ..),
                ) => {
                    if au == bu || bu == &Unit::None {
                        Some(Value::Numeric(av - bv, au.clone(), true))
                    } else if au == &Unit::None {
                        Some(Value::Numeric(av - bv, bu.clone(), true))
                    } else if let Some(scale) = bu.scale_to(au) {
                        Some(Value::Numeric(
                            (av.value - bv.value * scale).into(),
                            au.clone(),
                            true,
                        ))
                    } else {
                        None
                    }
                }
                // Note: This very special case should probably be much
                // more general.
                (&Value::UnicodeRange(..), &Value::Literal(..)) => {
                    Some(Value::List(
                        vec![
                            a.clone(),
                            Value::UnaryOp(
                                Operator::Minus,
                                Box::new(b.clone()),
                            ),
                        ],
                        ListSeparator::Space,
                        false,
                    ))
                }
                _ => None,
            },
            Operator::Multiply => {
                if let (
                    &Value::Numeric(ref a, ref au, ..),
                    &Value::Numeric(ref b, ref bu, ..),
                ) = (&a, &b)
                {
                    if bu == &Unit::None {
                        Some(Value::Numeric(a * b, au.clone(), true))
                    } else if au == &Unit::None {
                        Some(Value::Numeric(a * b, bu.clone(), true))
                    } else if bu == &Unit::Percent {
                        Some(Value::Numeric(
                            a * b * Rational::new(1, 100),
                            au.clone(),
                            true,
                        ))
                    } else if au == &Unit::Percent {
                        Some(Value::Numeric(
                            a * b * Rational::new(1, 100),
                            bu.clone(),
                            true,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Operator::Div => {
                if a.is_calculated() || b.is_calculated() {
                    match (&a, &b) {
                        (
                            &Value::Color(ref a, _),
                            &Value::Numeric(ref bn, Unit::None, ..),
                        ) => {
                            let bn = bn.value;
                            Some(Value::Color(a / bn, None))
                        }
                        (
                            &Value::Numeric(ref av, ref au, ..),
                            &Value::Numeric(ref bv, ref bu, ..),
                        ) => {
                            if bv.is_zero() {
                                None
                            } else if bu == &Unit::None {
                                Some(Value::Numeric(
                                    av / bv,
                                    au.clone(),
                                    true,
                                ))
                            } else if au == bu {
                                Some(Value::Numeric(
                                    av / bv,
                                    Unit::None,
                                    true,
                                ))
                            } else if let Some(scale) = bu.scale_to(&au) {
                                Some(Value::Numeric(
                                    av / &(bv * &scale.into()),
                                    Unit::None,
                                    true,
                                ))
                            } else {
                                None
                            }
                        }
                        //_ => None,
                        (a, b) => Some(Value::BinOp(
                            Box::new(a.clone()),
                            false, // *space1,
                            Operator::Div,
                            false, // *space2,
                            Box::new(b.clone()),
                        )),
                    }
                } else {
                    None
                }
            }
            Operator::Modulo => match (&a, &b) {
                (
                    &Value::Numeric(ref av, ref au, ..),
                    &Value::Numeric(ref bv, ref bu, ..),
                ) => {
                    if au == bu && !bv.is_zero() {
                        Some(Value::Numeric(av % bv, Unit::None, true))
                    } else if bu == &Unit::None && !bv.is_zero() {
                        Some(Value::Numeric(av % bv, au.clone(), true))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Operator::Not => panic!("not is a unary operator only"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "{}",
            match *self {
                Operator::And => "and",
                Operator::Or => "or",
                Operator::Equal => "==",
                Operator::NotEqual => "!=",
                Operator::Greater => ">",
                Operator::GreaterE => ">=",
                Operator::Lesser => "<",
                Operator::LesserE => "<=",
                Operator::Plus => "+",
                Operator::Minus => "-",
                Operator::Multiply => "*",
                Operator::Modulo => "%",
                Operator::Div => "/",
                Operator::Not => "not",
            }
        )
    }
}
