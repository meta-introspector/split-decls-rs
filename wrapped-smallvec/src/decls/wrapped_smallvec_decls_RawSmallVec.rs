use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Either a stack array with `length <= N` or a heap array
/// whose pointer and capacity are stored here.
///
/// We store a `NonNull<T>` instead of a `*mut T`, so that
/// niche-optimization can be performed and the type is covariant
/// with respect to `T`.
#[repr(C)]
pub union RawSmallVec<T, const N: usize> {
    inline: ManuallyDrop<MaybeUninit<[T; N]>>,
    heap: (NonNull<T>, usize),
}
