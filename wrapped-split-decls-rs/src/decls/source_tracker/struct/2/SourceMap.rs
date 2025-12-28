use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Struct to store source mapping for a token stream"] # [derive (Debug , Clone)] pub struct SourceMap { pub mappings : HashMap < usize , SourceLocation > , }