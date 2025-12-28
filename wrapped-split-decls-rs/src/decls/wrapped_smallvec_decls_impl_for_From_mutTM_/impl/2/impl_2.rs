use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Clone , const M : usize , const N : usize > From < & mut [T ; M] > for SmallVec < T , N > { # [inline] fn from (slice : & mut [T ; M]) -> Self { Self :: from (slice as & [T]) } }
}