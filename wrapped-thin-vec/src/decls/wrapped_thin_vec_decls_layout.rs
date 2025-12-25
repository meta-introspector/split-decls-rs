use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Gets the layout necessary to allocate a `ThinVec<T>`
///
/// # Panics
///
/// Panics if the required size overflows `isize::MAX`.
fn layout<T>(cap: usize) -> Layout {
    unsafe { Layout::from_size_align_unchecked(alloc_size::<T>(cap), alloc_align::<T>()) }
}
