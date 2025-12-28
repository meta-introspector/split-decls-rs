use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] struct LmdfbMapping { nodes : HashMap < String , LatticeNode > , layers : HashMap < String , Vec < String > > , total_levels : u32 , weight_distribution : HashMap < u32 , f64 > , }
}