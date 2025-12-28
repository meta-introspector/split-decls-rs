use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeFrom < I > { type Output = core :: range :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeFrom { start : self . start . index () } } }