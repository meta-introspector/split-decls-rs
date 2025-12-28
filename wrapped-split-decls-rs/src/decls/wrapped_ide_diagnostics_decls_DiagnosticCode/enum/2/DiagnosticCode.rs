use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum DiagnosticCode { RustcHardError (& 'static str) , SyntaxError , RustcLint (& 'static str) , Clippy (& 'static str) , Ra (& 'static str , Severity) , }
}