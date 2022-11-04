use domain::{EmailAddress, Feature, FeatureName, User, UserFirstName, UserLastName};

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

#[derive()]
pub struct SearchFeatureRequestDTO {
    pub name: Option<FeatureName>,
}

#[derive()]
pub struct SearchFeatureResponseDTO {
    pub features: Vec<Feature>,
}

#[derive()]
pub struct AddFeatureRequestDTO {
    pub feature: Feature,
}

#[derive()]
pub struct AddFeatureResponseDTO;
