use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (rustc_expand_base_lib_macros :: Diagnostic)] # [diag (dummy_diag)] pub struct MyDiagnostic { # [primary_span] pub span : Span , }