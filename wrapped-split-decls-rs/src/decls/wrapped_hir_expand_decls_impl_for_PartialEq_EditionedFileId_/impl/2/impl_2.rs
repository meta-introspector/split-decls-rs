use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < EditionedFileId > for HirFileId { fn eq (& self , & other : & EditionedFileId) -> bool { * self == HirFileId :: from (other) } }
}