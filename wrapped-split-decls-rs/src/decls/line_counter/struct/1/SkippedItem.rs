use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct SkippedItem { pub item_type : String , pub reason : String , pub content_preview : String , }