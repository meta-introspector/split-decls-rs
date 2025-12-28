use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Full workflow executor that follows the entire split-decls-rs pipeline"] pub struct WorkflowExecutor { audit_log : Vec < String > , step_count : usize , debug_mode : bool , }
}