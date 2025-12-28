use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn unparse (file : & File) -> String { let mut p = Printer :: new () ; p . file (file) ; p . eof () }
}