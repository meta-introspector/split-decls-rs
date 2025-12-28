use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeTo < I > { type Output = ops :: RangeTo < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { .. self . end . index () } }
}