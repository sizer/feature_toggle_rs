use rand::{distributions::Standard, prelude::Distribution, Rng};

pub(crate) struct IdGenerator;

impl IdGenerator {
    pub(crate) fn gen<T>() -> T
    where
        Standard: Distribution<T>,
    {
        // TODO: need to gen unique id.
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}
