use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Gets the padding necessary for the array of a `ThinVec<T>`
fn padding<T>() -> usize {
    let alloc_align = alloc_align::<T>();
    let header_size = mem::size_of::<Header>();
    if alloc_align > header_size {
        if cfg!(feature = "gecko-ffi") {
            panic!(
                "nsTArray does not handle alignment above > {} correctly", header_size
            );
        }
        alloc_align - header_size
    } else {
        0
    }
}
