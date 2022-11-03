use std::fmt;

pub struct UserFirstName(String);

impl UserFirstName {
    pub fn new(first_name: impl ToString) -> Self {
        Self(first_name.to_string())
    }
}

impl fmt::Display for UserFirstName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
