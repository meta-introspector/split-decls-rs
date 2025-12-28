use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Deserialize)] pub struct RefactorMeta { pub name : String , pub output_format : String , }
}