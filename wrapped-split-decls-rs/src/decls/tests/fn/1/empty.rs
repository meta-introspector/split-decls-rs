use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn empty () { let mut sets = UnionFind :: < u32 > :: new (10) ; for i in 1 .. 10 { assert_eq ! (sets . find (i) , i) ; } }