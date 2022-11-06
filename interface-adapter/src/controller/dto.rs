use domain::{EmailAddress, Feature, FeatureName, User, UserFirstName, UserLastName};

pub struct SearchUserRequestDTO {
    pub email: Option<EmailAddress>,
    pub first_name: Option<UserFirstName>,
    pub last_name: Option<UserLastName>,
}

pub struct SearchUserResponseDTO {
    pub users: Vec<User>,
}

pub struct AddUserRequestDTO {
    pub user: User,
}

pub struct AddUserResponseDTO;

pub struct UpdateUserRequestDTO {
    pub email: EmailAddress,
    pub first_name: Option<UserFirstName>,
    pub last_name: Option<UserLastName>,
}

pub struct UpdateUserResponseDTO;

pub struct SearchFeatureRequestDTO {
    pub name: Option<FeatureName>,
}

pub struct SearchFeatureResponseDTO {
    pub features: Vec<Feature>,
}

pub struct AddFeatureRequestDTO {
    pub feature: Feature,
}

pub struct AddFeatureResponseDTO;

pub struct GetFeatureRequestDTO {
    pub user_id: domain::UserId,
}

pub struct GetFeatureResponseDTO {
    pub features: Vec<domain::Feature>,
}
