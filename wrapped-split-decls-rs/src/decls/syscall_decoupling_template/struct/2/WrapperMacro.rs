use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct WrapperMacro { pub name : String , pub trait_name : String , pub decoupling_strategy : DecouplingStrategy , }
}