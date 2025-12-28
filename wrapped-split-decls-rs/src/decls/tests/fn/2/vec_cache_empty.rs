use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [cfg (not (miri))] fn vec_cache_empty () { let cache : VecCache < u32 , u32 , u32 > = VecCache :: default () ; for key in 0 .. u32 :: MAX { assert ! (cache . lookup (& key) . is_none ()) ; } }