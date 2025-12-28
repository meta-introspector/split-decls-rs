use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct DeriveHelper { pub (crate) derive : MacroId , pub (crate) idx : u32 , }
}