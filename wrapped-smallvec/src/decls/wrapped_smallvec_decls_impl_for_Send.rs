use serde::{Deserialize, Serialize};
use std::collections::HashMap;
unsafe impl<T, const N: usize> Send for IntoIter<T, N>
where
    T: Send,
{}
