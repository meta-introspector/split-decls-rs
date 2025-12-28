use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " The predefined `HKEY_LOCAL_MACHINE` registry key."] pub const LOCAL_MACHINE : & Key = & Key (HKEY_LOCAL_MACHINE) ;
}