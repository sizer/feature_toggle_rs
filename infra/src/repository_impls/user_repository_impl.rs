use std::cell;

use domain::{MyError, MyErrorType, MyResult, User, UserRepository};

use crate::{id_generator::IdGenerator, persistence::yaml::user_yaml_storage::UserYamlStorage};

#[derive()]
pub struct UserRepositoryImpl {
    users: cell::RefCell<Vec<User>>,
}

impl Default for UserRepositoryImpl {
    fn default() -> Self {
        let storage = UserYamlStorage::new();
        let users = storage.load();
        Self {
            users: cell::RefCell::new(users),
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn list(&self) -> Vec<User> {
        self.users.borrow().clone()
    }

    fn create(&self, name: domain::UserName, email: domain::EmailAddress) -> MyResult<()> {
        self.users.borrow_mut().push(domain::User::new(
            domain::UserId::new(IdGenerator::gen()),
            name,
            email,
        ));
        self.save();
        Ok(())
    }

    fn update(&self, user: User) -> MyResult<()> {
        let idx = self
            .users
            .borrow()
            .iter()
            .enumerate()
            .find_map(|(idx, u)| if u.id() == user.id() { Some(idx) } else { None })
            .ok_or_else(|| {
                MyError::new(
                    MyErrorType::NotFound,
                    format!("Passed user ID `{:?}` does not exist", user.id()),
                )
            })?;

        let _old = std::mem::replace(&mut self.users.borrow_mut()[idx], user);
        self.save();
        Ok(())
    }
}

impl UserRepositoryImpl {
    fn save(&self) {
        let storage = UserYamlStorage::new();
        storage.save(&self.users.borrow())
    }
}
