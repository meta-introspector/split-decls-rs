use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_noncopy () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) ; } }