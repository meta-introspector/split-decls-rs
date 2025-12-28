use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_typed_arena_drop_count () { let counter = Cell :: new (0) ; { let arena : TypedArena < DropCounter < '_ > > = TypedArena :: default () ; for _ in 0 .. 100 { arena . alloc (DropCounter { count : & counter }) ; } } ; assert_eq ! (counter . get () , 100) ; }