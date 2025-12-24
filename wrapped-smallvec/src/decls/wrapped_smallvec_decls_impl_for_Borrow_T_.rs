use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, const N: usize> Borrow<[T]> for SmallVec<T, N> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}
