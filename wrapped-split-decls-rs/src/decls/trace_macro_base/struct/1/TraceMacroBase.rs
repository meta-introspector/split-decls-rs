use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (CustomDiagnostic)] pub struct TraceMacroBase { pub span : Span , }
}