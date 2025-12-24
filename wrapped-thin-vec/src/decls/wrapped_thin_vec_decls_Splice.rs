use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A splicing iterator for `ThinVec`.
///
/// This struct is created by [`ThinVec::splice`][].
/// See its documentation for more.
///
/// # Example
///
/// ```
/// use thin_vec::thin_vec;
///
/// let mut v = thin_vec![0, 1, 2];
/// let new = [7, 8];
/// let iter: thin_vec::Splice<_> = v.splice(1.., new);
/// ```
#[derive(Debug)]
pub struct Splice<'a, I: Iterator + 'a> {
    drain: Drain<'a, I::Item>,
    replace_with: I,
}
