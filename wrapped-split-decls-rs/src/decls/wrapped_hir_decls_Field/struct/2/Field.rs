use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Field { pub (crate) parent : VariantDef , pub (crate) id : LocalFieldId , }
}