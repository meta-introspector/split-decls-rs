use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] struct CodePattern { pattern : String , locations : Vec < String > , similarity_score : f64 , }
}