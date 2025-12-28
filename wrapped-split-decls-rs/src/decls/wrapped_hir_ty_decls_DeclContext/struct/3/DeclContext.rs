use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Provides context for checking patterns in declarations. More specifically this"] # [doc = " allows us to infer array types if the pattern is irrefutable and allows us to infer"] # [doc = " the size of the array. See issue rust-lang/rust#76342."] # [derive (Debug , Copy , Clone)] pub (crate) struct DeclContext { pub (crate) origin : DeclOrigin , }
}