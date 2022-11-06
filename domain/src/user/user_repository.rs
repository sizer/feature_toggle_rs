use crate::{error::MyResult, EmailAddress, UserName};

use super::User;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait UserRepository {
    fn list(&self) -> Vec<User>;

    /// # Failures
    ///
    /// - `MyErrorType::Duplicate` : when user with given ID already exists.
    fn create(&self, name: UserName, email: EmailAddress) -> MyResult<()>;

    /// # Failures
    ///
    /// - `MyErrorType::NotFound` : when user with given ID (inside User) does not exist.
    fn update(&self, user: User) -> MyResult<()>;
}
