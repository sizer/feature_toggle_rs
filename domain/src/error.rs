use std::{error, fmt};

use self::error_type::MyErrorType;

pub mod error_type;

pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug)]
pub struct MyError {
    typ: MyErrorType,
    desc: String,
}

impl MyError {
    pub fn new(typ: MyErrorType, desc: impl ToString) -> Self {
        Self {
            typ,
            desc: desc.to_string(),
        }
    }
}

impl error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.typ, self.desc)
    }
}
