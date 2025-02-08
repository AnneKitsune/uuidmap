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
        if let Some(index) = self.map.get_mut(&key) {
            *index = self.data.len();
            self.data.push(value);
        } else {
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
        let mut table: Table<i32> = Table::default();
        let key = 123;
        table.add_with_key(key, 42);
        assert_eq!(table.get(key), Some(&42));

        table.add_with_key(key, 50);
        assert_eq!(table.get(key), Some(&50));
    }

    #[test]
    fn test_count() {
        let mut table: Table<i32> = Table::default();
        assert_eq!(table.count(), 0);

        table.add(42);
        assert_eq!(table.count(), 1);

        table.add_with_key(123, 50);
        assert_eq!(table.count(), 2);
    }

    #[test]
    fn test_remove() {
        let mut table: Table<i32> = Table::default();
        let key = table.add(42);
        assert_eq!(table.get(key), Some(&42));

        let value = table.remove(key);
        assert_eq!(value, Some(42));
        assert_eq!(table.get(key), None);
    }

    #[test]
    fn test_values() {
        let mut table: Table<i32> = Table::default();
        table.add(42);
        table.add_with_key(123, 50);

        let values: Vec<&i32> = table.values().collect();
        assert_eq!(values, vec![&42, &50]);
    }

    #[test]
    fn test_edge_cases() {
        let mut table: Table<i32> = Table::default();
        let key = 123;
        table.add_with_key(key, 42);
        assert_eq!(table.get(key), Some(&42));

        table.remove(key);
        assert_eq!(table.get(key), None);

        table.add_with_key(key, 50);
        assert_eq!(table.get(key), Some(&50));
    }
}
