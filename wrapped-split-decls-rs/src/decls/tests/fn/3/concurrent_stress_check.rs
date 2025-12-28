use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn concurrent_stress_check () { let cache : VecCache < u32 , u32 , u32 > = VecCache :: default () ; std :: thread :: scope (| s | { for idx in 0 .. 100 { let cache = & cache ; s . spawn (move | | { cache . complete (idx , idx , idx) ; }) ; } }) ; for idx in 0 .. 100 { assert_eq ! (cache . lookup (& idx) , Some ((idx , idx))) ; } }
}