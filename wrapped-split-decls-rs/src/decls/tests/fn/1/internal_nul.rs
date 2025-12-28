use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [should_panic] fn internal_nul () { let _ = SmallCStr :: new ("abcd\0def") ; }
}