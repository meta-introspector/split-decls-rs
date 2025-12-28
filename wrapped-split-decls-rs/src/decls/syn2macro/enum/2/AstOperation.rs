use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " AST operation types for security checking"] # [derive (Debug , Clone)] pub enum AstOperation { ParseItem , TransformItem , GenerateCode , FileAccess (String) , NetworkAccess , SystemCall (String) , }