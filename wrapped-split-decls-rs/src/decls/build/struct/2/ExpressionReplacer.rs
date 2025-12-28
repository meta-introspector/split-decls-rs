use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct ExpressionReplacer < 'a > { function_name : & 'a str , old_snippet : syn :: Expr , new_snippet : syn :: Expr , replaced_count : usize , }