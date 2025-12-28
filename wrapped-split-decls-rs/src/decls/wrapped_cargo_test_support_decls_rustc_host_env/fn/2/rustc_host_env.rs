use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " The host triple suitable for use in a cargo environment variable (uppercased)."] pub fn rustc_host_env () -> String { rustc_host () . to_uppercase () . replace ('-' , "_") }
}