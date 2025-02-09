use fxhash::FxHashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Table<T> {
    // Contains a map of uuid to data vector index.
    map: FxHashMap<u128, usize>,
    data: Vec<T>,
    reverse: Vec<u128>,
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self {
            map: FxHashMap::<u128, usize>::default(),
            data: vec![],
            reverse: vec![],
        }
    }
}

impl<T> Table<T> {
    /// Remove an element using it's key.
    pub fn remove(&mut self, key: u128) -> Option<T> {
        if let Some(index) = self.map.remove(&key) {
            // Swap-remove from both data and reverse
            let value = self.data.swap_remove(index);
            // key that got moved to index
            let pre_move_index = self.reverse[self.reverse.len() - 1];
            self.reverse.swap_remove(index);

            // if what we removed was not the last element, update the index
            if index < self.reverse.len() {
                *self.map.get_mut(&pre_move_index).unwrap() = index;
            }
            return Some(value);
        }
        None
    }

    /// Get the number of elements stored.
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Empty out everything.
    pub fn clear(&mut self) {
        self.data.clear();
        self.reverse.clear();
        self.map.clear();
    }

    /// Get an iterator over the contained values.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Return an iterator over keys.
    pub fn keys(&self) -> std::collections::hash_map::Keys<u128, usize> {
        self.map.keys()
    }

    /// Get a value by key.
    pub fn get(&self, key: u128) -> Option<&T> {
        if let Some(index) = self.map.get(&key) {
            // unsafe: index is valid. improves performance by 4-7% on my machine.
            Some(unsafe { self.data.get_unchecked(*index) })
        } else {
            None
        }
    }

    /// Get a value by key.
    pub fn get_mut(&mut self, key: u128) -> Option<&mut T> {
        if let Some(index) = self.map.get(&key) {
            // unsafe: index is valid. improves performance by 4-7% on my machine.
            Some(unsafe { self.data.get_unchecked_mut(*index) })
        } else {
            None
        }
    }

    /// Add a new value with random key.
    /// This is what you want to use 95% of the time.
    pub fn add(&mut self, value: T) -> u128 {
        let key = rand::rng().random();
        self.add_with_key(key, value);
        key
    }

    /// Add a new value with manual key. Usually used during deserialization.
    /// Might be used for performance reasons when using a Table as a Map.
    /// For example, a map KeyCode -> GameEvent.
    pub fn add_with_key(&mut self, key: u128, value: T) {
        self.remove(key);
        self.data.push(value);
        self.reverse.push(key);
        let index = self.data.len() - 1;
        self.map.insert(key, index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get() {
        let mut table: Table<i32> = Table::default(); // Specify type for empty_table
        let key = table.add(42);
        assert_eq!(table.get(key), Some(&42));
    }

    #[test]
    fn test_add_with_key() {
        let mut table: Table<i32> = Table::default(); // Specify type for empty_table
        let key = 123;
        table.add_with_key(key, 42);
        assert_eq!(table.get(key), Some(&42));
    }

    #[test]
    fn test_remove() {
        let mut table: Table<i32> = Table::default(); // Specify type for empty_table
        let key = table.add(42);
        assert_eq!(table.remove(key), Some(42));
        assert_eq!(table.get(key), None);
    }

    #[test]
    fn test_count() {
        let mut table: Table<i32> = Table::default(); // Specify type for empty_table
        assert_eq!(table.count(), 0);
        table.add(42);
        assert_eq!(table.count(), 1);

        let key = table.add(24);
        table.remove(key);
        assert_eq!(table.count(), 1);
    }

    #[test]
    fn test_values() {
        let mut table: Table<i32> = Table::default(); // Specify type for empty_table
        table.add(42);
        table.add(24);
        let values: Vec<_> = table.values().collect();
        assert_eq!(values, vec![&42, &24]);
    }

    #[test]
    fn test_edge_cases() {
        let mut table: Table<i32> = Table::default(); // Specify type for empty_table
                                                      // Adding and removing elements
        let key1 = table.add(1);
        let key2 = table.add(2);
        assert_eq!(table.get(key1), Some(&1));
        assert_eq!(table.get(key2), Some(&2));
        assert_eq!(table.remove(key1), Some(1));
        assert_eq!(table.get(key1), None);
        assert_eq!(table.count(), 1);

        // Adding with specific key
        let key3 = 999;
        table.add_with_key(key3, 3);
        assert_eq!(table.get(key3), Some(&3));

        // Removing non-existent key
        assert_eq!(table.remove(998), None);

        // Getting non-existent key
        assert_eq!(table.get(998), None);

        // Counting empty table
        let empty_table: Table<i32> = Table::default(); // Specify type for empty_table
        assert_eq!(empty_table.count(), 0);
    }
}
