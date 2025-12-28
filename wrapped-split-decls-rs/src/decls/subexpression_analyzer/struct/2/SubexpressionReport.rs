use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] struct SubexpressionReport { pattern : String , count : usize , emoji_hash : String , locations : Vec < String > , }