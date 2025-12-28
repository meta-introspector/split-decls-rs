use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (CustomSubdiagnostic)] pub struct TraceMacroNote { pub span : Span , pub message : String , }
}