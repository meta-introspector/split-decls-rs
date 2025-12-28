use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_noncopy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) }) }