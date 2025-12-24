use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compare two string-like types for case-less equality, using unicode folding.
///
/// Equivalent to `UniCase::new(left) == UniCase::new(right)`.
///
/// Note: This will perform a scan for ASCII characters before doing the
/// the comparison. See `UniCase` for more information.
#[inline]
pub fn eq<S: AsRef<str> + ?Sized>(left: &S, right: &S) -> bool {
    UniCase::new(left) == UniCase::new(right)
}
