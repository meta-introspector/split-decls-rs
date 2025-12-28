use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct FieldId { pub parent : VariantId , pub local_id : LocalFieldId , }
}