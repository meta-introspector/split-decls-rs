use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_copy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | arena . alloc (Point { x : 1 , y : 2 , z : 3 })) }