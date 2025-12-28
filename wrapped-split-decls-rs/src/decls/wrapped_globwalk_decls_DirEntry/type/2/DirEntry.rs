use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " A directory entry."] # [doc = ""] # [doc = " This is the type of value that is yielded from the iterators defined in this crate."] pub type DirEntry = walkdir :: DirEntry ;
}