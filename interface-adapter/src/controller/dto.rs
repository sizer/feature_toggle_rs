use domain::{EmailAddress, User, UserFirstName, UserLastName};

#[derive()]
pub struct SearchUserRequestDTO {
    pub email: Option<EmailAddress>,
    pub first_name: Option<UserFirstName>,
    pub last_name: Option<UserLastName>,
}

#[derive()]
pub struct SearchUserResponseDTO {
    pub users: Vec<User>,
}

#[derive()]
pub struct AddUserRequestDTO {
    pub user: User,
}

#[derive()]
pub struct AddUserResponseDTO;

#[derive()]
pub struct UpdateUserRequestDTO {
    pub email: EmailAddress,
    pub first_name: Option<UserFirstName>,
    pub last_name: Option<UserLastName>,
}

#[derive()]
pub struct UpdateUserResponseDTO;
