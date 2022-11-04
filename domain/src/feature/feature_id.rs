#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FeatureId(u64);

impl FeatureId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
