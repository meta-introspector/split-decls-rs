use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_copy_nonarena (b : & mut Bencher) { b . iter (| | { let _ : Box < _ > = Box :: new (Point { x : 1 , y : 2 , z : 3 }) ; }) }