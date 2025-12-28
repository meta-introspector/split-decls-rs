use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn default_fn_name (base : Symbol) -> String { format ! ("__rdl_{base}") }