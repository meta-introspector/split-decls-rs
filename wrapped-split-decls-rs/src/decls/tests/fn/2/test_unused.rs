use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_unused () { let arena : TypedArena < Point > = TypedArena :: default () ; assert ! (arena . chunks . borrow () . is_empty ()) ; }