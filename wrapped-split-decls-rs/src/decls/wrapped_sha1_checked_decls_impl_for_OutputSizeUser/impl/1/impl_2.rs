use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl OutputSizeUser for Sha1 { type OutputSize = U20 ; }
}