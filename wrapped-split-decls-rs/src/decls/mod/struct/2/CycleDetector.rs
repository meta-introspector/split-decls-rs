use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " This `TriColorVisitor` looks for back edges in a graph, which indicate that a cycle exists."] pub struct CycleDetector ;
}