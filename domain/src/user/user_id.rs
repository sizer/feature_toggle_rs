#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
