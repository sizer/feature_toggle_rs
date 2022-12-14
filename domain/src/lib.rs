///! Domain layer.
mod email_address;
mod error;
mod feature;
mod repositories;
mod user;

pub use email_address::EmailAddress;
pub use error::{error_type::MyErrorType, MyError, MyResult};
pub use feature::{
    feature_distribution_strategy::FeatureDistributionStrategy, feature_id::FeatureId,
    feature_name::FeatureName, feature_repository::FeatureRepository, Feature,
};
pub use repositories::Repositories;
pub use user::{
    user_id::UserId,
    user_name::{user_first_name::UserFirstName, user_last_name::UserLastName, UserName},
    user_repository::UserRepository,
    User,
};

#[cfg(feature = "mock")]
pub use feature::feature_repository::MockFeatureRepository;
#[cfg(feature = "mock")]
pub use user::user_repository::MockUserRepository;
