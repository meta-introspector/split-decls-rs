use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn inclusive_end < T : Idx > (domain : usize , range : impl RangeBounds < T >) -> Option < u32 > { let end = match range . end_bound () { Bound :: Included (end) => end . index () as u32 , Bound :: Excluded (end) => end . index () . checked_sub (1) ? as u32 , Bound :: Unbounded => domain . checked_sub (1) ? as u32 , } ; Some (end) }
}