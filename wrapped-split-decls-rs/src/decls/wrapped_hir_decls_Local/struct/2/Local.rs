use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A single local definition."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct Local { pub (crate) parent : DefWithBodyId , pub (crate) binding_id : BindingId , }
}