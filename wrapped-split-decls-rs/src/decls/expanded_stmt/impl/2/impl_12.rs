use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl CfgFalseExpandable for ExpandedStmt { fn expand_cfg_false < D : CfgFalseReporterContext > (& mut self , collector : & mut D , pos : usize , span : Span) { self . visit_stmt_attrs (| attrs | { attrs . remove (pos) ; }) ; } }
}