use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait CfgFalseExpandable : Sized { fn expand_cfg_false < D : CfgFalseReporterContext > (& mut self , collector : & mut D , pos : usize , span : Span) ; }
}