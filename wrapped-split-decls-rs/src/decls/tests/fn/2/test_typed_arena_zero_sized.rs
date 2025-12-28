use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_typed_arena_zero_sized () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (()) ; } }
}