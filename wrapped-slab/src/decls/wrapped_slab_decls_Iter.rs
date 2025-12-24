use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator over the values stored in the `Slab`
pub struct Iter<'a, T> {
    entries: iter::Enumerate<slice::Iter<'a, Entry<T>>>,
    len: usize,
}
