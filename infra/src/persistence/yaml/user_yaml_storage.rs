use std::{fs, io::Write, path};

use domain::User;

#[derive()]
pub(crate) struct UserYamlStorage;

impl UserYamlStorage {
    pub(crate) fn new() -> Self {
        let slf = Self;

        if !Self::yaml_path().exists() {
            let yml = slf.serialize_users(&vec![]);
            Self::write_to_yaml(&yml)
        }
        slf
    }

    pub(crate) fn load(&self) -> Vec<User> {
        let yml = fs::read_to_string(Self::yaml_path()).expect("Error on reading YAML");
        self.deserialize_users(&yml)
    }

    pub(crate) fn save(&self, users: &Vec<User>) {
        let yml = self.serialize_users(&users);
        Self::write_to_yaml(&yml)
    }

    fn serialize_users(&self, users: &Vec<User>) -> String {
        serde_yaml::to_string(&users).expect("serialization error")
    }

    fn deserialize_users(&self, yml: &str) -> Vec<User> {
        serde_yaml::from_str(&yml).expect("deserialization error")
    }

    fn yaml_path() -> path::PathBuf {
        path::PathBuf::from("users.yml")
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
