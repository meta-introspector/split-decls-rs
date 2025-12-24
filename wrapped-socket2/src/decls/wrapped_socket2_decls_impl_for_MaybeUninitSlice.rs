use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> MaybeUninitSlice<'a> {
    /// Creates a new `MaybeUninitSlice` wrapping a byte slice.
    ///
    /// # Panics
    ///
    /// Panics on Windows if the slice is larger than 4GB.
    pub fn new(buf: &'a mut [MaybeUninit<u8>]) -> MaybeUninitSlice<'a> {
        MaybeUninitSlice(sys::MaybeUninitSlice::new(buf))
    }
}
