use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Clone, const N: usize> From<&mut [T]> for SmallVec<T, N> {
    #[inline]
    fn from(slice: &mut [T]) -> Self {
        Self::from(slice as &[T])
    }
}
