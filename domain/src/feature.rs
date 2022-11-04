use self::{feature_id::FeatureId, feature_name::FeatureName};

pub mod feature_id;
pub mod feature_name;
pub mod feature_repository;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Feature {
    id: FeatureId,
    name: FeatureName,
}

impl Feature {
    pub fn new(id: FeatureId, name: FeatureName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &FeatureId {
        &self.id
    }

    pub fn name(&self) -> &FeatureName {
        &self.name
    }
}
