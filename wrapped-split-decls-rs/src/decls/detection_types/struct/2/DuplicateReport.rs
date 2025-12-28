use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Serialize)] pub struct DuplicateReport { pub fingerprint : String , pub line_count : usize , pub blocks : Vec < DuplicateBlock > , }
}