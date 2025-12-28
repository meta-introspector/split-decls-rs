use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_typed_arena_drop_small_count () { DROP_COUNTER . with (| c | c . set (0)) ; { let arena : TypedArena < SmallDroppable > = TypedArena :: default () ; for _ in 0 .. 100 { arena . alloc (SmallDroppable) ; } } ; assert_eq ! (DROP_COUNTER . with (| c | c . get ()) , 100) ; }