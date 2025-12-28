use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I , T > IntoSliceIdx < I , [T] > for ops :: RangeFull { type Output = ops :: RangeFull ; # [inline] fn into_slice_idx (self) -> Self :: Output { self } }