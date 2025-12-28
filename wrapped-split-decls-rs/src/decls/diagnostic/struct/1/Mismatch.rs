use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Mismatch { slug_name : String , crate_name : String , slug_prefix : String , }
}