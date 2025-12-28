use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The defs which have a body."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum DefWithBody { Function (Function) , Static (Static) , Const (Const) , Variant (Variant) , }