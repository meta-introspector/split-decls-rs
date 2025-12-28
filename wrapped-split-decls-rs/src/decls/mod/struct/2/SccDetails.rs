use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Information about an invidividual SCC node."] struct SccDetails { # [doc = " For this SCC, the range of `all_successors` where its"] # [doc = " successors can be found."] range : Range < usize > , }
}