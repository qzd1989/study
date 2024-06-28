use crate::client;

pub struct Threads {
    pub id: i32,
}

impl Threads {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

pub fn generate() -> client::People {
    client::People {
        name: "hello".to_string(),
    }
}
