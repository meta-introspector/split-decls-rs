use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl IntoDiagArg for Level { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . to_cmd_flag ())) } }