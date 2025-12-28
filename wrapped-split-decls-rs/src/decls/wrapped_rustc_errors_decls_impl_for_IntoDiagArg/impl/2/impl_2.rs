use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl IntoDiagArg for Level { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: from (self . to_string ())) } }
}