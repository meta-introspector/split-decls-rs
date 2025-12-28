use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeToInclusive < I > { type Output = ops :: RangeToInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ..= self . end . index () } }
}