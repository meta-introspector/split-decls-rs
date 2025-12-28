use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Eq)] pub struct CodeBlock { pub start_byte : usize , pub end_byte : usize , pub start_line : usize , pub end_line : usize , pub fingerprint : String , pub ast_lines : usize , }