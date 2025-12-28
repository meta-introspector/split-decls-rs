use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn scope_join () { let mut v : Vec < i32 > = (0 .. 256) . rev () . collect () ; quick_sort (& mut v) ; assert ! (is_sorted (& v)) ; }