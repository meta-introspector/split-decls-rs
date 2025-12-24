use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> From<Box<[T]>> for ThinVec<T> {
    /// Convert a boxed slice into a vector by transferring ownership of
    /// the existing heap allocation.
    ///
    /// **NOTE:** unlike `std`, this must reallocate to change the layout!
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    ///
    /// let b: Box<[i32]> = thin_vec![1, 2, 3].into_iter().collect();
    /// assert_eq!(ThinVec::from(b), thin_vec![1, 2, 3]);
    /// ```
    fn from(s: Box<[T]>) -> Self {
        Vec::from(s).into_iter().collect()
    }
}
