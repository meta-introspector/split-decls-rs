use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (rustc_expand_base_lib_macros :: Subdiagnostic)] # [note (dummy_note)] pub struct MySubdiagnostic { # [primary_span] pub span : Span , pub message : String , }
}