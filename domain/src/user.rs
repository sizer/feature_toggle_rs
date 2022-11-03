use self::{user_id::UserId, user_name::UserName};

pub mod user_id;
pub mod user_name;
pub mod user_repository;

pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }
}
