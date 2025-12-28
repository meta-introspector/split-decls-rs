use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn range_count_is_correct () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . count () , 3) ; }
}