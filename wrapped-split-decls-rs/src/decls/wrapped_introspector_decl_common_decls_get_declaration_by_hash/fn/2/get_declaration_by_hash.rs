use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn get_declaration_by_hash (hash : & str) -> Option < DeclInfo > { DECL_REGISTRY . lock () . ok () ? . by_hash . get (hash) . and_then (| & idx | DECL_REGISTRY . lock () . ok () ? . declarations . get (idx) . cloned ()) }