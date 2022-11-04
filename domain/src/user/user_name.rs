pub mod user_first_name;
pub mod user_last_name;

use std::fmt;

use self::{user_first_name::UserFirstName, user_last_name::UserLastName};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UserName {
    first_name: UserFirstName,
    last_name: UserLastName,
}

impl UserName {
    pub fn new(first_name: UserFirstName, last_name: UserLastName) -> Self {
        Self {
            first_name,
            last_name,
        }
    }

    pub fn first_name(&self) -> &UserFirstName {
        &self.first_name
    }

    pub fn last_name(&self) -> &UserLastName {
        &self.last_name
    }
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}
