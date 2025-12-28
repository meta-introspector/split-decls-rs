use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct InstantiatedStruct < 'db > { pub (crate) inner : Struct , pub (crate) args : GenericArgs < 'db > , }
}