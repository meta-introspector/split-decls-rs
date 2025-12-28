use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " --- Configuration Structures for Edit Jobs ---"] # [derive (Debug , Deserialize)] pub struct EditJobConfig { pub edits : Vec < EditJob > , }
}