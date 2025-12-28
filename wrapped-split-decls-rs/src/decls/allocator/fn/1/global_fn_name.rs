use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn global_fn_name (base : Symbol) -> String { format ! ("__rust_{base}") }