use std::fmt;

pub struct UserLastName(String);

impl UserLastName {
    pub fn new(last_name: impl ToString) -> Self {
        Self(last_name.to_string())
    }
}

impl fmt::Display for UserLastName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
