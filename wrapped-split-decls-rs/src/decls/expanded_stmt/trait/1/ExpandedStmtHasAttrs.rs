use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait ExpandedStmtHasAttrs { fn visit_stmt_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) ; }