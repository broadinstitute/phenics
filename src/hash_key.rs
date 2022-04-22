use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub(crate) struct HashKey {
    hash1: u64,
    hash2: u64,
}

const SALT: [u64; 8] = [126, 39, 22, 47, 75, 207, 77, 5];

impl From<&str> for HashKey {
    fn from(string: &str) -> Self {
        let mut hasher1 = DefaultHasher::new();
        string.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        let mut hasher2 = DefaultHasher::new();
        SALT.hash(&mut hasher2);
        string.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        HashKey { hash1, hash2 }
    }
}

