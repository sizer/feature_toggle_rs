use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EmailAddress {
    user: String,
    domain: String,
}

impl EmailAddress {
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

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}
