use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn sparse_inserts () { let cache : VecCache < u32 , u8 , u32 > = VecCache :: default () ; let end = if cfg ! (target_pointer_width = "64") && cfg ! (target_os = "linux") { 31 } else { 25 } ; for shift in 0 .. end { let key = 1u32 << shift ; cache . complete (key , shift , key) ; assert_eq ! (cache . lookup (& key) , Some ((shift , key))) ; } }
}