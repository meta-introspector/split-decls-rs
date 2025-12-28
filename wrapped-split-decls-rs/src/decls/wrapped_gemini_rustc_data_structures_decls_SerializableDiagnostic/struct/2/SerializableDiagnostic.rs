use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SerializableDiagnostic { pub message : String , pub level : String , pub span : Option < SerializableSpan > , }
}