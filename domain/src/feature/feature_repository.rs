use crate::{error::MyResult, Feature, FeatureDistributionStrategy, FeatureName};

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait FeatureRepository {
    fn list(&self) -> Vec<Feature>;

    /// # Failures
    ///
    /// - `MyErrorType::Duplicate` : when feature with given ID already exists.
    fn create(&self, name: FeatureName, strategy: FeatureDistributionStrategy) -> MyResult<()>;

    /// # Failures
    ///
    /// - `MyErrorType::NotFound` : when feature with given ID (inside Feature) does not exist.
    fn update(&self, feature: Feature) -> MyResult<()>;
}
