// in-memory user repository

use std::collections::HashMap;
use crate::repository_interface::{Repository, User, UserRepository};

pub struct InMemoryUserRepository {
    store: HashMap<String, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Repository<String, User> for InMemoryUserRepository {
    fn save(&mut self, id: String, entity: User) {
        self.store.insert(id, entity);
    }

    fn find_by_id(&self, id: &String) -> Option<&User> {
        self.store.get(id)
    }

    fn find_all(&self) -> Vec<&User> {
        self.store.values().collect()
    }

    fn delete(&mut self, id: &String) {
        self.store.remove(id);
    }
}

impl UserRepository for InMemoryUserRepository {}

