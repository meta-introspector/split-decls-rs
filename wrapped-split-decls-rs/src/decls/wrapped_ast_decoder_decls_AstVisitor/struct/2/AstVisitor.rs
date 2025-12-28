use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Helper struct to traverse the AST and collect statistics"] struct AstVisitor { stats : AstStatistics , # [allow (dead_code)] file_path : PathBuf , }
}