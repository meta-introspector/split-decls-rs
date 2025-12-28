use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct ConflictReport { crate_name : String , dependency_name : String , conflict_details : Vec < String > , }
}