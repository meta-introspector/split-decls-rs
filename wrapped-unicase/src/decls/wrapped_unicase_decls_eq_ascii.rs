use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compare two string-like types for case-less equality, ignoring ASCII case.
///
/// Equivalent to `Ascii::new(left) == Ascii::new(right)`.
#[inline]
pub fn eq_ascii<S: AsRef<str> + ?Sized>(left: &S, right: &S) -> bool {
    Ascii(left) == Ascii(right)
}
