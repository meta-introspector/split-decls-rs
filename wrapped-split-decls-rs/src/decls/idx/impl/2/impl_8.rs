use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T > IntoSliceIdx < I , [T] > for I { type Output = usize ; # [inline] fn into_slice_idx (self) -> Self :: Output { self . index () } }
}