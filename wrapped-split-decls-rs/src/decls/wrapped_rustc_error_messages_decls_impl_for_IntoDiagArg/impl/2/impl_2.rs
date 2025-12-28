use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl IntoDiagArg for DiagArgValue { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self } }