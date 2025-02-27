use std::sync::Mutex;
use std::collections::HashMap;

use crate::structs::{DID, User, Post, Follow};

pub struct Storage {
    users: Mutex<HashMap<DID, User>>,
    posts: Mutex<HashMap<String, Post>>,
    follows: Mutex<Vec<Follow>>,
    key_pairs: Mutex<HashMap<DID, String>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            posts: Mutex::new(HashMap::new()),
            follows: Mutex::new(Vec::new()),
            key_pairs: Mutex::new(HashMap::new()),
        }
    }
}
