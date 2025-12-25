use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The metadata associated with a [`KnownLayout`] type.
#[doc(hidden)]
pub trait PointerMetadata: Copy + Eq + Debug {
    /// Constructs a `Self` from an element count.
    ///
    /// If `Self = ()`, this returns `()`. If `Self = usize`, this returns
    /// `elems`. No other types are currently supported.
    fn from_elem_count(elems: usize) -> Self;
    /// Computes the size of the object with the given layout and pointer
    /// metadata.
    ///
    /// # Panics
    ///
    /// If `Self = ()`, `layout` must describe a sized type. If `Self = usize`,
    /// `layout` must describe a slice DST. Otherwise, `size_for_metadata` may
    /// panic.
    ///
    /// # Safety
    ///
    /// `size_for_metadata` promises to only return `None` if the resulting size
    /// would not fit in a `usize`.
    fn size_for_metadata(self, layout: DstLayout) -> Option<usize>;
}
