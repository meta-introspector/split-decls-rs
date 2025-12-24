use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Clone> From<&[T]> for ThinVec<T> {
    /// Allocate a `ThinVec<T>` and fill it by cloning `s`'s items.
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    ///
    /// assert_eq!(ThinVec::from(&[1, 2, 3][..]), thin_vec![1, 2, 3]);
    /// ```
    fn from(s: &[T]) -> ThinVec<T> {
        s.iter().cloned().collect()
    }
}
