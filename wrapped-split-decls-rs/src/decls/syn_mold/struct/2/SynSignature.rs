use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Extracted signature of syn usage"] # [derive (Debug , Clone)] pub struct SynSignature { pub operation : String , pub input_types : Vec < String > , pub output_types : Vec < String > , pub complexity_score : f64 , pub dependencies : Vec < String > , }
}