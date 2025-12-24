use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "parsing", feature = "proc-macro"))]
mod parse_macro_input;
