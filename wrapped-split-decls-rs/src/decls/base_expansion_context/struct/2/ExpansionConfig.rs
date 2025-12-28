use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ExpansionConfig < 'a > { pub crate_name : Symbol , pub features : & 'a Features , pub recursion_limit : Limit , pub trace_mac : bool , pub should_test : bool , pub span_debug : bool , pub proc_macro_backtrace : bool , }