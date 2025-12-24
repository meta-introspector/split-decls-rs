use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> Ord for ThinVec<T>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &ThinVec<T>) -> Ordering {
        self[..].cmp(&other[..])
    }
}
