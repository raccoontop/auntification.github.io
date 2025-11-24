use std::borrow::Cow;
use std::collections::HashMap;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

struct HashMapStorage<K, V> {
    inner: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash, V> HashMapStorage<K, V> {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }
}

impl<K: Eq + std::hash::Hash + Clone, V> Storage<K, V> for HashMapStorage<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.inner.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
}

struct UserRepositoryStatic<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    fn new(storage: S) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

struct UserRepositoryDynamic {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDynamic {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

fn main() {
    println!("Lab3 Part1 is ready.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_repo() {
        let storage = HashMapStorage::new();
        let mut repo = UserRepositoryStatic::new(storage);

        let user = User { id: 1, email: "a@example.com".into(), activated: true };

        repo.add(user);
        assert!(repo.get(1).is_some());

        repo.remove(1);
        assert!(repo.get(1).is_none());
    }

    #[test]
    fn test_dynamic_repo() {
        let storage = HashMapStorage::new();
        let mut repo = UserRepositoryDynamic::new(Box::new(storage));

        let user = User { id: 2, email: "b@example.com".into(), activated: false };

        repo.add(user);
        assert!(repo.get(2).is_some());

        repo.remove(2);
        assert!(repo.get(2).is_none());
    }
}

