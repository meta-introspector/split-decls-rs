use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct ErrorItem { pub item_type : String , pub error_message : String , pub content_preview : String , }