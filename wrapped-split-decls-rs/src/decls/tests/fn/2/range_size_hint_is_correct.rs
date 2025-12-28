use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn range_size_hint_is_correct () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . size_hint () , (3 , Some (3))) ; }
}