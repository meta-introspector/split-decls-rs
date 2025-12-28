use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum BindingMode { Move , Ref (Mutability) , }
}