use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (CustomSubdiagnostic)] pub struct TraceMacroNote { pub span : Span , pub message : String , }