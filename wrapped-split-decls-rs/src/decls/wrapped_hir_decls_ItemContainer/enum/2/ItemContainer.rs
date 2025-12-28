use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ItemContainer { Trait (Trait) , Impl (Impl) , Module (Module) , ExternBlock (ExternBlock) , Crate (Crate) , }
}