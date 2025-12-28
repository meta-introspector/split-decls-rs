use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
pub const RAW_IDENT_ERR : & str = "`${concat(..)}` currently does not support raw identifiers" ;
}