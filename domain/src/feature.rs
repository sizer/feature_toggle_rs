pub mod feature_distribution_strategy;
pub mod feature_id;
pub mod feature_name;
pub mod feature_repository;

use self::{
    feature_distribution_strategy::FeatureDistributionStrategy, feature_id::FeatureId,
    feature_name::FeatureName,
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Feature {
    id: FeatureId,
    name: FeatureName,
    strategy: FeatureDistributionStrategy,
}

impl Feature {
    pub fn new(id: FeatureId, name: FeatureName, strategy: FeatureDistributionStrategy) -> Self {
        Self { id, name, strategy }
    }

    pub fn id(&self) -> &FeatureId {
        &self.id
    }

    pub fn name(&self) -> &FeatureName {
        &self.name
    }

    pub fn strategy(&self) -> &FeatureDistributionStrategy {
        &&self.strategy
    }
}
