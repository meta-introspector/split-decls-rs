use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [cfg (host_os = "windows")] const RUSTVERSION : Version = include ! (concat ! (env ! ("OUT_DIR") , "\\version.expr")) ;
}