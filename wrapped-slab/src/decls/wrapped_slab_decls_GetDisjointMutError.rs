use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq)]
/// The error type returned by [`Slab::get_disjoint_mut`].
pub enum GetDisjointMutError {
    /// An index provided was not associated with a value.
    IndexVacant,
    /// An index provided was out-of-bounds for the slab.
    IndexOutOfBounds,
    /// Two indices provided were overlapping.
    OverlappingIndices,
}
