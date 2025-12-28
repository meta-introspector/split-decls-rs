use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_typed_arena_clear (b : & mut Bencher) { let mut arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; arena . clear () ; }) }