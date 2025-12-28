use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] pub struct SynCache { cache : HashMap < String , FileCache > , cache_file : String , }
}