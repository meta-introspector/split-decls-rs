use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [salsa_macros :: input (debug)] pub struct FileText { # [returns (ref)] pub text : Arc < str > , pub file_id : vfs :: FileId , }