use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [must_use] pub fn get_compiler_version () -> Option < String > { let compiler = std :: option_env ! ("RUSTC") . unwrap_or ("rustc") ; get_output (compiler , & ["-V"]) }