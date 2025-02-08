use nohash_hasher::BuildNoHashHasher;
// Uuid -> vector idx
type UuidMap = HashMap<u128, usize, BuildNoHashHasher<u128>>;

pub struct Table<T> {
    // Contains a map of uuid to data vector index.
    map: UuidMap,
    data: Vec<T>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        let map = UuidMap::with_capacity_and_hasher(16, BuildNoHashHasher::default());
        Self {
            map,
            data: vec![],
        }
    }

    // remove, count, values -> iterator, get, add(value) -> key, add_with_key(key, value)
    // remove must use swapRemove in the vec and update the map
}

