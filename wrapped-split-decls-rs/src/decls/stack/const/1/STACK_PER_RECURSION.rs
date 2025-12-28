use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [cfg (target_os = "aix")] const STACK_PER_RECURSION : usize = 16 * 1024 * 1024 ;
}