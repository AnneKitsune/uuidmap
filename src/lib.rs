use rand::Rng;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Table<T> {
    // Contains a map of uuid to data vector index.
    map: HashMap<u128, usize>,
    data: Vec<T>,
}

impl<T> Table<T> {
    pub fn remove(&mut self, key: u128) -> Option<T> {
        if let Some(index) = self.map.remove(&key) {
            if index < self.data.len() {
                let value = self.data.swap_remove(index);
                if index != self.data.len() {
                    // Update the map if the removed element was not the last one
                    if let Some(last_key) = self
                        .map
                        .iter_mut()
                        .find(|&(_, &mut idx)| idx == self.data.len())
                    {
                        *last_key.1 = index;
                    }
                }
                return Some(value);
            }
        }
        None
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn get(&self, key: u128) -> Option<&T> {
        if let Some(index) = self.map.get(&key) {
            self.data.get(*index)
        } else {
            None
        }
    }

    pub fn add(&mut self, value: T) -> u128 {
        let key = rand::rng().random(); // Use thread_rng to generate a random key
        self.add_with_key(key, value);
        key
    }

    pub fn add_with_key(&mut self, key: u128, value: T) {
        if !self.map.contains_key(&key) {
            self.data.push(value);
            let index = self.data.len() - 1;
            self.map.insert(key, index);
        }
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
