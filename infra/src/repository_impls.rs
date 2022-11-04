use domain::Repositories;

use self::user_repository_impl::UserRepositoryImpl;

pub(crate) mod user_repository_impl;

#[derive(Default)]
pub(crate) struct RepositoryImpls {
    user_repo: UserRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
