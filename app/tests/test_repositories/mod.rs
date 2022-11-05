use domain::{MockFeatureRepository, MockUserRepository, Repositories};

pub struct TestRepositories {
    user_repo: MockUserRepository,
    feature_repo: MockFeatureRepository,
}
impl Repositories for TestRepositories {
    type UserRepo = MockUserRepository;
    type FeatureRepo = MockFeatureRepository;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repo
    }

    fn feature_repository(&self) -> &Self::FeatureRepo {
        &self.feature_repo
    }
}

impl TestRepositories {
    pub fn new(
        user_repo: Option<MockUserRepository>,
        feature_repo: Option<MockFeatureRepository>,
    ) -> Self {
        Self {
            user_repo: user_repo.unwrap_or(MockUserRepository::new()),
            feature_repo: feature_repo.unwrap_or(MockFeatureRepository::new()),
        }
    }
}
