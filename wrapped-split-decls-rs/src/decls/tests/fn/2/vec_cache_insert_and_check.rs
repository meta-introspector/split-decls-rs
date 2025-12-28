use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn vec_cache_insert_and_check () { let cache : VecCache < u32 , u32 , u32 > = VecCache :: default () ; cache . complete (0 , 1 , 2) ; assert_eq ! (cache . lookup (& 0) , Some ((1 , 2))) ; }