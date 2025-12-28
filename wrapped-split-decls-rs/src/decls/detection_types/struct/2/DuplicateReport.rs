use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Serialize)] pub struct DuplicateReport { pub fingerprint : String , pub line_count : usize , pub blocks : Vec < DuplicateBlock > , }