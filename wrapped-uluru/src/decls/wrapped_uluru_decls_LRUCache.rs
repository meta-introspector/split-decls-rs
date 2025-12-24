use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A LRU cache using a statically-sized array for storage.
///
/// `LRUCache` uses a fixed-capacity array for storage. It provides `O(1)` insertion, and `O(n)`
/// lookup.
///
/// All items are stored inline within the `LRUCache`, so it does not impose any heap allocation or
/// indirection.  A linked list is used to record the cache order, so the items themselves do not
/// need to be moved when the order changes.  (This is important for speed if the items are large.)
///
/// # Example
///
/// ```
/// use uluru::LRUCache;
///
/// struct MyValue {
///     id: u32,
///     name: &'static str,
/// }
///
/// // A cache with a capacity of three.
/// type MyCache = LRUCache<MyValue, 3>;
///
/// // Create an empty cache, then insert some items.
/// let mut cache = MyCache::default();
/// cache.insert(MyValue { id: 1, name: "Mercury" });
/// cache.insert(MyValue { id: 2, name: "Venus" });
/// cache.insert(MyValue { id: 3, name: "Earth" });
///
/// // Use the `find` method to retrieve an item from the cache.
/// // This also "touches" the item, marking it most-recently-used.
/// let item = cache.find(|x| x.id == 1);
/// assert_eq!(item.unwrap().name, "Mercury");
///
/// // If the cache is full, inserting a new item evicts the least-recently-used item:
/// cache.insert(MyValue { id: 4, name: "Mars" });
/// assert!(cache.find(|x| x.id == 2).is_none());
/// ```
#[derive(Debug, Clone)]
pub struct LRUCache<T, const N: usize> {
    /// The most-recently-used entry is at index `head`. The entries form a linked list, linked to
    /// each other by indices within the `entries` array.  After an entry is added to the array,
    /// its index never changes, so these links are never invalidated.
    entries: ArrayVec<Entry<T>, N>,
    /// Index of the first entry. If the cache is empty, ignore this field.
    head: u16,
    /// Index of the last entry. If the cache is empty, ignore this field.
    tail: u16,
}
