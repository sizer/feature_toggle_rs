mod fixture;
mod test_repositories;

use app::UseCase;
use domain::Feature;
use domain::MockFeatureRepository;
use domain::MockUserRepository;
use domain::User;
use fixture::feature::FeatureFixture;
use fixture::user::UserFixture;
use test_repositories::TestRepositories;

#[test]
fn test_with_no_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo.expect_list().returning(|| vec![]);
    let mut user_repo = MockUserRepository::new();
    user_repo.expect_list().returning(|| vec![User::fx1()]);

    let repositories = TestRepositories::new(Some(user_repo), Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(User::fx1().id()).unwrap(), vec![]);
}

#[test]
fn test_with_public_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo
        .expect_list()
        .returning(|| vec![Feature::fx_public_1()]);
    let mut user_repo = MockUserRepository::new();
    user_repo.expect_list().returning(|| vec![User::fx1()]);

    let repositories = TestRepositories::new(Some(user_repo), Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(
        use_case.get_features(User::fx1().id()).unwrap(),
        vec![Feature::fx_public_1()]
    );
}

#[test]
fn test_with_private_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo
        .expect_list()
        .returning(|| vec![Feature::fx_private_1()]);
    let mut user_repo = MockUserRepository::new();
    user_repo.expect_list().returning(|| vec![User::fx1()]);

    let repositories = TestRepositories::new(Some(user_repo), Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(User::fx1().id()).unwrap(), vec![]);
}

#[test]
fn test_with_ab_test_50_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo
        .expect_list()
        .returning(|| vec![Feature::fx_ab_test(50)]);
    let mut user_repo = MockUserRepository::new();
    user_repo
        .expect_list()
        .returning(|| vec![User::fx1(), User::fx2()]);

    let repositories = TestRepositories::new(Some(user_repo), Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(User::fx1().id()).unwrap(), vec![]);
    assert_eq!(
        use_case.get_features(User::fx2().id()).unwrap(),
        vec![Feature::fx_ab_test(50)]
    );
}

#[test]
fn test_with_ab_test_33_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo
        .expect_list()
        .returning(|| vec![Feature::fx_ab_test(33)]);
    let mut user_repo = MockUserRepository::new();
    user_repo
        .expect_list()
        .returning(|| vec![User::fx1(), User::fx2(), User::fx3()]);

    let repositories = TestRepositories::new(Some(user_repo), Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(User::fx1().id()).unwrap(), vec![]);
    assert_eq!(use_case.get_features(User::fx2().id()).unwrap(), vec![]);
    assert_eq!(
        use_case.get_features(User::fx3().id()).unwrap(),
        vec![Feature::fx_ab_test(33)]
    );
}

#[test]
fn test_with_multiple_features() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo.expect_list().returning(|| {
        vec![
            Feature::fx_public_1(),
            Feature::fx_private_1(),
            Feature::fx_ab_test(50),
        ]
    });
    let mut user_repo = MockUserRepository::new();
    user_repo
        .expect_list()
        .returning(|| vec![User::fx1(), User::fx2()]);

    let repositories = TestRepositories::new(Some(user_repo), Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(
        use_case.get_features(User::fx1().id()).unwrap(),
        vec![Feature::fx_public_1()]
    );
    assert_eq!(
        use_case.get_features(User::fx2().id()).unwrap(),
        vec![Feature::fx_public_1(), Feature::fx_ab_test(50)]
    );
}
