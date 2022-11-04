use std::cell;

use domain::{Feature, FeatureRepository, MyError, MyErrorType, MyResult};

use crate::persistence::yaml::feature_yaml_storage::FeatureYamlStorage;

#[derive()]
pub(crate) struct FeatureRepositoryImpl {
    features: cell::RefCell<Vec<Feature>>,
}

impl Default for FeatureRepositoryImpl {
    fn default() -> Self {
        let storage = FeatureYamlStorage::new();
        let feature = storage.load();
        Self {
            features: cell::RefCell::new(feature),
        }
    }
}

impl FeatureRepository for FeatureRepositoryImpl {
    fn list(&self) -> Vec<Feature> {
        self.features.borrow().clone()
    }

    fn create(&self, feature: Feature) -> MyResult<()> {
        if self
            .features
            .borrow()
            .iter()
            .any(|f| f.id() == feature.id() || f.name() == feature.name())
        {
            Err(MyError::new(
                MyErrorType::Duplicate,
                format!("Duplicate feature: {:?}", feature),
            ))
        } else {
            self.features.borrow_mut().push(feature);
            self.save();
            Ok(())
        }
    }

    fn update(&self, feature: Feature) -> MyResult<()> {
        let idx = self
            .features
            .borrow()
            .iter()
            .enumerate()
            .find_map(|(idx, f)| {
                if f.id() == feature.id() {
                    Some(idx)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                MyError::new(
                    MyErrorType::NotFound,
                    format!("Passed feature ID `{:?}` does not exist", feature.id()),
                )
            })?;

        let _old = std::mem::replace(&mut self.features.borrow_mut()[idx], feature);
        self.save();
        Ok(())
    }
}

impl FeatureRepositoryImpl {
    fn save(&self) {
        let storage = FeatureYamlStorage::new();
        storage.save(&self.features.borrow())
    }
}
