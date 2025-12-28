use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " The predefined `HKEY_CLASSES_ROOT` registry key."] pub const CLASSES_ROOT : & Key = & Key (HKEY_CLASSES_ROOT) ;
}