use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait CfgFalseReporter { fn report_cfg_false < N : HasAttrs + HasNodeId > (& mut self , node : & mut N , attr_span : Span , attr_pos : usize) ; }