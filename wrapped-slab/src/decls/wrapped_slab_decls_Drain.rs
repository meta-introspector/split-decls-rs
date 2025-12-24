use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A draining iterator for `Slab`
pub struct Drain<'a, T> {
    inner: vec::Drain<'a, Entry<T>>,
    len: usize,
}
