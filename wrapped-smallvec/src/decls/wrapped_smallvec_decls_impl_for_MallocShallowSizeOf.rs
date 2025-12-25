use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "malloc_size_of")]
impl<T, const N: usize> MallocShallowSizeOf for SmallVec<T, N> {
    fn shallow_size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
        if self.spilled() { unsafe { ops.malloc_size_of(self.as_ptr()) } } else { 0 }
    }
}
