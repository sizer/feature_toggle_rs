pub trait FeatureFixture {
    fn fx_public_1() -> domain::Feature {
        domain::Feature::new(
            domain::FeatureId::new(1),
            domain::FeatureName::new("PublicFeature1"),
            domain::FeatureDistributionStrategy::Public,
        )
    }

    fn fx_private_1() -> domain::Feature {
        domain::Feature::new(
            domain::FeatureId::new(1),
            domain::FeatureName::new("PrivateFeature1"),
            domain::FeatureDistributionStrategy::Private,
        )
    }

    fn fx_ab_test(percent: u8) -> domain::Feature {
        domain::Feature::new(
            domain::FeatureId::new(1),
            domain::FeatureName::new(format!("AbTestFeature{}", percent)),
            domain::FeatureDistributionStrategy::ABTest(percent),
        )
    }
}

impl FeatureFixture for domain::Feature {}
