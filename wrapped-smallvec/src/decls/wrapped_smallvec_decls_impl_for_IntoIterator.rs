use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, T, const N: usize> IntoIterator for &'a mut SmallVec<T, N> {
    type IntoIter = core::slice::IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
