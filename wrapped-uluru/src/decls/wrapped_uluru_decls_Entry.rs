use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An entry in an `LRUCache`.
#[derive(Debug, Clone)]
struct Entry<T> {
    val: T,
    /// Index of the previous entry. If this entry is the head, ignore this field.
    prev: u16,
    /// Index of the next entry. If this entry is the tail, ignore this field.
    next: u16,
}
