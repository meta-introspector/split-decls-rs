use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct RustcInfo { verbose_version : String , host : String , }
}