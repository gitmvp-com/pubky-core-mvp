//! In-memory storage backend
//!
//! Provides a simple HashMap-based storage implementation.
//! In production, this would be replaced with LMDB or another persistent store.

use pubky_common::PublicKey;
use std::collections::HashMap;
use std::sync::RwLock;

/// In-memory key-value storage
pub struct Storage {
    data: RwLock<HashMap<(PublicKey, String), Vec<u8>>>,
}

impl Storage {
    /// Create a new empty storage
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    /// Store a value at the given public key and path
    pub fn put(&self, public_key: PublicKey, path: String, value: Vec<u8>) {
        let mut data = self.data.write().unwrap();
        data.insert((public_key, path), value);
        tracing::debug!("Stored data for {} at path", public_key);
    }

    /// Retrieve a value at the given public key and path
    pub fn get(&self, public_key: &PublicKey, path: &str) -> Option<Vec<u8>> {
        let data = self.data.read().unwrap();
        data.get(&(*public_key, path.to_string())).cloned()
    }

    /// Delete a value at the given public key and path
    pub fn delete(&self, public_key: &PublicKey, path: &str) -> bool {
        let mut data = self.data.write().unwrap();
        data.remove(&(*public_key, path.to_string())).is_some()
    }

    /// List all paths for a given public key with a prefix
    pub fn list(&self, public_key: &PublicKey, prefix: &str) -> Vec<String> {
        let data = self.data.read().unwrap();
        data.keys()
            .filter(|(pk, path)| pk == public_key && path.starts_with(prefix))
            .map(|(_, path)| path.clone())
            .collect()
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pubky_common::Keypair;

    #[test]
    fn test_storage_operations() {
        let storage = Storage::new();
        let keypair = Keypair::random();
        let public_key = keypair.public_key();
        let path = "test/data.txt".to_string();
        let value = b"Hello, World!".to_vec();

        // Put
        storage.put(public_key, path.clone(), value.clone());

        // Get
        let retrieved = storage.get(&public_key, &path);
        assert_eq!(retrieved, Some(value));

        // Delete
        assert!(storage.delete(&public_key, &path));
        assert_eq!(storage.get(&public_key, &path), None);
    }

    #[test]
    fn test_storage_list() {
        let storage = Storage::new();
        let keypair = Keypair::random();
        let public_key = keypair.public_key();

        storage.put(public_key, "app/file1.txt".to_string(), vec![1]);
        storage.put(public_key, "app/file2.txt".to_string(), vec![2]);
        storage.put(public_key, "other/file3.txt".to_string(), vec![3]);

        let app_files = storage.list(&public_key, "app/");
        assert_eq!(app_files.len(), 2);
        assert!(app_files.contains(&"app/file1.txt".to_string()));
        assert!(app_files.contains(&"app/file2.txt".to_string()));
    }
}
