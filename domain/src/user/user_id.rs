#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    // TODO: unwrap primitive value without this dummy method is better.
    pub fn v(&self) -> u64 {
        self.0
    }
}
