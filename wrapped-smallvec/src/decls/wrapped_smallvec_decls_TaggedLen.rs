use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Vec guarantees that its length is always less than [`isize::MAX`] in *bytes*.
///
/// For a non ZST, this means that the length is less than `isize::MAX` objects, which implies we
/// have at least one free bit we can use. We use the least significant bit for the tag. And store
/// the length in the `usize::BITS - 1` most significant bits.
///
/// For a ZST, we never use the heap, so we just store the length directly.
#[repr(transparent)]
#[derive(Clone, Copy)]
struct TaggedLen(usize);
