use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct ConstParam { pub (crate) id : ConstParamId , }
}