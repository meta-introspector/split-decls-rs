use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl AstVisitor { fn new (file_path : PathBuf) -> Self { Self { stats : AstStatistics :: default () , file_path , } } fn increment_node_type_count (& mut self , node_type : & str) { * self . stats . node_type_counts . entry (node_type . to_string ()) . or_insert (0) += 1 ; } }