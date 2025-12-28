use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeFrom < I > { type Output = ops :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: RangeFrom { start : self . start . index () } } }