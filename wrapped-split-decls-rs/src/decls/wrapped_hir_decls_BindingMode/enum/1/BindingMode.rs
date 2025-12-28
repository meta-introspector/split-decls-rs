use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum BindingMode { Move , Ref (Mutability) , }