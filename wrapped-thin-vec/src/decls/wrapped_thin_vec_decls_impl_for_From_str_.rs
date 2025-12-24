use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<&str> for ThinVec<u8> {
    /// Allocate a `ThinVec<u8>` and fill it with a UTF-8 string.
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    ///
    /// assert_eq!(ThinVec::from("123"), thin_vec![b'1', b'2', b'3']);
    /// ```
    fn from(s: &str) -> ThinVec<u8> {
        From::from(s.as_bytes())
    }
}
