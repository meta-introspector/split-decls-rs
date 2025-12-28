use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeInclusive < I > { type Output = ops :: RangeInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: RangeInclusive :: new (self . start () . index () , self . end () . index ()) } }