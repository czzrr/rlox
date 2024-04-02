use std::fmt::Display;

pub enum Value {
    Double(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Double(n) => write!(f, "{}", n),
        }
    }
}
