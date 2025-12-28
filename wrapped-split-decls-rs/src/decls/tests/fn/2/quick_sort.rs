use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn quick_sort < T : PartialOrd + Send > (v : & mut [T]) { if v . len () <= 1 { return ; } let mid = partition (v) ; let (lo , hi) = v . split_at_mut (mid) ; join (| | quick_sort (lo) , | | quick_sort (hi)) ; }
}