use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Filesystem operations oracle"] pub trait FileSystemOracle { fn audit_read () -> Result < () , String > ; fn audit_write () -> Result < () , String > ; fn check_path_safety (path : & str) -> bool ; }
}