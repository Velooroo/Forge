pub struct Repository {
    id: u32,
    name: String,
    description: String,
    owner_id: u32,
}

impl Repository {
    pub fn new(id: u32, name: String, description: String, owner_id: u32) -> Self {
        Self {
            id,
            name,
            description,
            owner_id,
        }
    }
}
