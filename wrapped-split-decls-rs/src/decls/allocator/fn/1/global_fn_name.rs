use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn global_fn_name (base : Symbol) -> String { format ! ("__rust_{base}") }
}