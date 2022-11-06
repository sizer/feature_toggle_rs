use std::{fs, io::Write, path};

use domain::{Feature, MyError, MyErrorType, MyResult};

#[derive()]
pub(crate) struct FeatureYamlStorage;

impl FeatureYamlStorage {
    pub(crate) fn new() -> Self {
        let slf = Self;

        if !Self::yaml_path().exists() {
            let yml = slf.serialize_features(&vec![]);
            Self::write_to_yaml(&yml).expect("Initialize YamlStorage failed.")
        }
        slf
    }

    pub(crate) fn load(&self) -> Vec<Feature> {
        let yml = fs::read_to_string(Self::yaml_path()).expect("Error on reading YAML");
        self.deserialize_features(&yml)
    }

    pub(crate) fn save(&self, features: &Vec<Feature>) -> MyResult<()> {
        let yml = self.serialize_features(&features);
        Self::write_to_yaml(&yml)
    }

    fn serialize_features(&self, features: &Vec<Feature>) -> String {
        serde_yaml::to_string(&features).expect("serialization error")
    }

    fn deserialize_features(&self, yml: &str) -> Vec<Feature> {
        serde_yaml::from_str(&yml).expect("deserialization error")
    }

    fn yaml_path() -> path::PathBuf {
        path::PathBuf::from("features.yml")
    }

    fn write_to_yaml(yml: &str) -> MyResult<()> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(Self::yaml_path())
            .map_err(|e| {
                MyError::new(
                    MyErrorType::PersistFailed,
                    format!("Error on opening YAML file: {}", e),
                )
            })?;

        write!(file, "{}", yml).map_err(|e| {
            MyError::new(
                MyErrorType::PersistFailed,
                format!("Error on writing to YAML file: {}", e),
            )
        })?;

        file.flush().map_err(|e| {
            MyError::new(
                MyErrorType::PersistFailed,
                format!("Error on flusing: {}", e),
            )
        })
    }
}
