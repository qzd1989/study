use crate::pool;

pub struct People {
    pub name: String,
}

pub fn generate() -> pool::Threads {
    pool::Threads::new(12)
}
