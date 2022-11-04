use std::{fs, io::Write, path};

use domain::Feature;

#[derive()]
pub(crate) struct FeatureYamlStorage;

impl FeatureYamlStorage {
    pub(crate) fn new() -> Self {
        let slf = Self;

        if !Self::yaml_path().exists() {
            let yml = slf.serialize_features(&vec![]);
            Self::write_to_yaml(&yml)
        }
        slf
    }

    pub(crate) fn load(&self) -> Vec<Feature> {
        let yml = fs::read_to_string(Self::yaml_path()).expect("Error on reading YAML");
        self.deserialize_features(&yml)
    }

    pub(crate) fn save(&self, features: &Vec<Feature>) {
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

    fn write_to_yaml(yml: &str) {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(Self::yaml_path())
            .expect("Error on opening YAML file");

        write!(f, "{}", yml).expect("Error on writing to YAML file");
        f.flush().expect("ERror on flusing");
    }
}
