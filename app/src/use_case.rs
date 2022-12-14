use domain::{
    EmailAddress, Feature, FeatureName, FeatureRepository, MyError, MyErrorType, MyResult,
    Repositories, User, UserFirstName, UserLastName, UserName, UserRepository,
};

pub struct UseCase<'r, R: Repositories> {
    user_repo: &'r R::UserRepo,
    feature_repo: &'r R::FeatureRepo,
}

impl<'r, R: Repositories> UseCase<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            user_repo: repositories.user_repository(),
            feature_repo: repositories.feature_repository(),
        }
    }

    pub fn search_users(
        &self,
        first_name: Option<&UserFirstName>,
        last_name: Option<&UserLastName>,
        email: Option<&EmailAddress>,
    ) -> Vec<User> {
        fn name_eq(a: &str, b: &str) -> bool {
            a.to_lowercase() == b.to_lowercase()
        }

        if first_name.is_none() && last_name.is_none() && email.is_none() {
            vec![]
        } else {
            let users = self.user_repo.list();
            let users = users
                .into_iter()
                .filter(|user| {
                    first_name
                        .map(|f_name| {
                            name_eq(&f_name.to_string(), &user.name().first_name().to_string())
                        })
                        .unwrap_or_else(|| true)
                        && last_name
                            .map(|l_name| {
                                name_eq(&l_name.to_string(), &user.name().last_name().to_string())
                            })
                            .unwrap_or_else(|| true)
                        && email.map(|em| em == user.email()).unwrap_or_else(|| true)
                })
                .collect();
            users
        }
    }

    pub fn add_user(&self, name: domain::UserName, email: domain::EmailAddress) -> MyResult<()> {
        self.user_repo.create(name, email)
    }

    pub fn update_user_by_email(
        &self,
        email: &EmailAddress,
        first_name: Option<UserFirstName>,
        last_name: Option<UserLastName>,
    ) -> MyResult<()> {
        let users = self.user_repo.list();

        let user = users
            .into_iter()
            .find(|user| user.email() == email)
            .ok_or_else(|| {
                MyError::new(
                    MyErrorType::NotFound,
                    format!("User with email address '{}' does not exist", email),
                )
            })?;

        let new_user_name = UserName::new(
            first_name.unwrap_or_else(|| user.name().first_name().clone()),
            last_name.unwrap_or_else(|| user.name().last_name().clone()),
        );
        let new_user = User::new(user.id().clone(), new_user_name, user.email().clone());

        self.user_repo.update(new_user)
    }

    pub fn search_features(&self, name: Option<&FeatureName>) -> Vec<Feature> {
        fn name_eq(a: &str, b: &str) -> bool {
            a.to_lowercase() == b.to_lowercase()
        }

        if name.is_none() {
            vec![]
        } else {
            let features = self.feature_repo.list();
            let features = features
                .into_iter()
                .filter(|a_feature| {
                    name.map(|n| name_eq(&n.to_string(), &a_feature.name().to_string()))
                        .unwrap_or_else(|| true)
                })
                .collect();
            features
        }
    }

    pub fn add_feature(
        &self,
        name: domain::FeatureName,
        strategy: domain::FeatureDistributionStrategy,
    ) -> MyResult<()> {
        self.feature_repo.create(name, strategy)
    }

    pub fn get_features(&self, user_id: &domain::UserId) -> MyResult<Vec<domain::Feature>> {
        let user: Option<domain::User> = self
            .user_repo
            .list()
            .iter()
            .filter(|u| u.id() == user_id)
            .map(|u| u.clone())
            .next();

        match user {
            Some(u) => Ok(self
                .feature_repo
                .list()
                .into_iter()
                .filter(|f| match f.strategy() {
                    &domain::FeatureDistributionStrategy::Public => true,
                    &domain::FeatureDistributionStrategy::Private => false,
                    &domain::FeatureDistributionStrategy::ABTest(p) => {
                        u.id().v() % (100 / p as u64) == 0
                    }
                })
                .collect()),
            None => Err(MyError::new(
                MyErrorType::NotFound,
                format!("User not found. UserId: {:?}", user_id),
            )),
        }
    }
}
