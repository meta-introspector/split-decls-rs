use serde::{Deserialize, Serialize};
use std::collections::HashMap;
unsafe impl<T, const N: usize> Sync for IntoIter<T, N>
where
    T: Sync,
{}
