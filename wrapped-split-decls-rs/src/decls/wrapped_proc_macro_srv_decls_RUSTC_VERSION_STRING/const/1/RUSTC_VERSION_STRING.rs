use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
pub const RUSTC_VERSION_STRING : & str = env ! ("RUSTC_VERSION") ;
}