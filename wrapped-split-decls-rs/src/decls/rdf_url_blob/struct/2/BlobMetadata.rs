use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct BlobMetadata { pub timestamp : String , pub system_version : String , pub macro_count : usize , pub export_capabilities : Vec < String > , }
}