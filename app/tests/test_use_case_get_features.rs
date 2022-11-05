mod test_repositories;

use app::UseCase;
use domain::MockFeatureRepository;
use test_repositories::TestRepositories;

#[test]
fn test_with_no_feature() {
    let mut feature_repo = MockFeatureRepository::new();
    feature_repo.expect_list().returning(|| vec![]);

    let repositories = TestRepositories::new(None, Some(feature_repo));
    let use_case = UseCase::new(&repositories);

    assert_eq!(use_case.get_features(domain::UserId::new(1)), vec![]);
}
