use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > ExpansionConfig < 'a > { pub fn default (crate_name : Symbol , features : & 'a Features) -> ExpansionConfig < 'a > { ExpansionConfig { crate_name , features , recursion_limit : Limit :: new (1024) , trace_mac : false , should_test : false , span_debug : false , proc_macro_backtrace : false , } } }