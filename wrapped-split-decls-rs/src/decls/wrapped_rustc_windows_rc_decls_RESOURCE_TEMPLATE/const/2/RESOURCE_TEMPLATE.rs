use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " The template for the Windows resource file."] const RESOURCE_TEMPLATE : & str = include_str ! ("../rustc.rc.in") ;
}