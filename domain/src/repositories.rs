use crate::{
    feature::feature_repository::FeatureRepository, user::user_repository::UserRepository,
};

/// To simplify declaring types on other layer, use this trait to aggregate.
/// For example, `XUseCase` has two repositories like `ARepository` and `BRepository`,
/// We can define `XUseCase<R: Repositories>` instead of `XUseCase<A: ARepository, B: BRepository>`.
pub trait Repositories {
    type UserRepo: UserRepository;
    type FeatureRepo: FeatureRepository;

    fn user_repository(&self) -> &Self::UserRepo;
    fn feature_repository(&self) -> &Self::FeatureRepo;
}
