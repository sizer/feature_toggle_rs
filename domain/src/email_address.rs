use std::fmt;

pub struct EmailAdress {
    user: String,
    domain: String,
}

impl EmailAdress {
    pub fn new(s: impl ToString) -> Self {
        // TODO validate s

        let s = s.to_string();
        let mut sp = s.split('@');
        Self {
            user: sp.next().expect("wrong email address format").to_string(),
            domain: sp.next().expect("wrong email address format").to_string(),
        }
    }
}

impl fmt::Display for EmailAdress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}
