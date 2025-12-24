use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "gecko-ffi")]
#[repr(C, align(8))]
struct AutoBuffer<T, const N: usize> {
    header: Header,
    buffer: mem::MaybeUninit<[T; N]>,
}
