use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Default , Serialize , Deserialize)] pub struct VariantStats { pub count : u64 , pub contexts : Vec < String > , pub dependencies : Vec < String > , pub patterns : Vec < String > , }