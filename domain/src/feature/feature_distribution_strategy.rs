#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FeatureDistributionStrategy {
    Public,
    Private,
    ABTest(u8),
}
