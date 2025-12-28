use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The rustc host such as `x86_64-unknown-linux-gnu`."] pub fn rustc_host () -> & 'static str { & rustc_info () . host }