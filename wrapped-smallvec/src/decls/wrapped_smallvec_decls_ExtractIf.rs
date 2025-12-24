use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "extract_if")]
/// An iterator which uses a closure to determine if an element should be removed.
///
/// Returned from [`SmallVec::extract_if`][1].
///
/// [1]: struct.SmallVec.html#method.extract_if
pub struct ExtractIf<'a, T, const N: usize, F>
where
    F: FnMut(&mut T) -> bool,
{
    vec: &'a mut SmallVec<T, N>,
    /// The index of the item that will be inspected by the next call to `next`.
    idx: usize,
    /// Elements at and beyond this point will be retained. Must be equal or smaller than `old_len`.
    end: usize,
    /// The number of items that have been drained (removed) thus far.
    del: usize,
    /// The original length of `vec` prior to draining.
    old_len: usize,
    /// The filter test predicate.
    pred: F,
}
