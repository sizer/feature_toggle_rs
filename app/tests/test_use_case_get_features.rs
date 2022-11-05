mod fixture;
mod test_repositories;

use app::UseCase;
use domain::Feature;
use domain::MockFeatureRepository;
use fixture::feature::FeatureFixture;
use test_repositories::TestRepositories;

#[test]
fn test_with_no_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo.expect_list().returning(|| vec![]);

    let repositories = TestRepositories::new(None, Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(domain::UserId::new(1)), vec![]);
}

#[test]
fn test_with_public_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo
        .expect_list()
        .returning(|| vec![Feature::fx_public_1()]);

    let repositories = TestRepositories::new(None, Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(
        use_case.get_features(domain::UserId::new(1)),
        vec![Feature::fx_public_1()]
    );
}

#[test]
fn test_with_private_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo
        .expect_list()
        .returning(|| vec![Feature::fx_private_1()]);

    let repositories = TestRepositories::new(None, Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(domain::UserId::new(1)), vec![]);
}

#[test]
fn test_with_ab_test_50_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo
        .expect_list()
        .returning(|| vec![Feature::fx_ab_test_50()]);

    let repositories = TestRepositories::new(None, Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(domain::UserId::new(1)), vec![]);
    assert_eq!(
        use_case.get_features(domain::UserId::new(2)),
        vec![Feature::fx_ab_test_50()]
    );
}

#[test]
fn test_with_multiple_features() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo.expect_list().returning(|| {
        vec![
            Feature::fx_public_1(),
            Feature::fx_private_1(),
            Feature::fx_ab_test_50(),
        ]
    });

    let repositories = TestRepositories::new(None, Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(
        use_case.get_features(domain::UserId::new(1)),
        vec![Feature::fx_public_1()]
    );
    assert_eq!(
        use_case.get_features(domain::UserId::new(2)),
        vec![Feature::fx_public_1(), Feature::fx_ab_test_50()]
    );
}
