use domain::Repositories;

use self::{
    feature_repository_impl::FeatureRepositoryImpl, user_repository_impl::UserRepositoryImpl,
};

pub mod feature_repository_impl;
pub mod user_repository_impl;

#[derive(Default)]
pub struct RepositoryImpls {
    user_repo: UserRepositoryImpl,
    feature_repo: FeatureRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;
    type FeatureRepo = FeatureRepositoryImpl;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }

    fn feature_repository(&self) -> &Self::FeatureRepo {
        &self.feature_repo
    }
}
