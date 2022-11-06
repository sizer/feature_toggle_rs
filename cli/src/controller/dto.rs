use domain::{Feature, FeatureName, User, UserFirstName, UserLastName};

pub struct SearchUserRequestDTO {
    pub email: Option<domain::EmailAddress>,
    pub first_name: Option<UserFirstName>,
    pub last_name: Option<UserLastName>,
}

pub struct SearchUserResponseDTO {
    pub users: Vec<User>,
}

pub struct AddUserRequestDTO {
    pub name: domain::UserName,
    pub email: domain::EmailAddress,
}

pub struct AddUserResponseDTO;

pub struct UpdateUserRequestDTO {
    pub email: domain::EmailAddress,
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
    pub name: FeatureName,
    pub strategy: domain::FeatureDistributionStrategy,
}

pub struct AddFeatureResponseDTO;

pub struct GetFeatureRequestDTO {
    pub user_id: domain::UserId,
}

pub struct GetFeatureResponseDTO {
    pub features: Vec<domain::Feature>,
}
