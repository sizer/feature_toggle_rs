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

    fn fx_ab_test_50() -> domain::Feature {
        domain::Feature::new(
            domain::FeatureId::new(1),
            domain::FeatureName::new("AbTestFeature50"),
            domain::FeatureDistributionStrategy::ABTest(50),
        )
    }
}

impl FeatureFixture for domain::Feature {}
