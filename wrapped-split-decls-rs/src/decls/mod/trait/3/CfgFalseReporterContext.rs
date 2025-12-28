use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait CfgFalseReporterContext { fn buffer_lint_unused_attribute < 'a > (& mut self , lint : Lint , span : Span , lint_node_id : NodeId , diag : impl Diagnostic < 'a > + 'a ,) ; fn get_unused_attribute_lint (& self , is_cfg : bool) -> Lint ; fn get_lint_node_id (& self) -> NodeId ; fn get_strip_unconfigured (& self) -> StripUnconfigured ; }
}