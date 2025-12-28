use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct SynOperation { pub name : String , pub input_types : Vec < String > , pub output_types : Vec < String > , pub complexity : f64 , pub dependencies : Vec < String > , }
}