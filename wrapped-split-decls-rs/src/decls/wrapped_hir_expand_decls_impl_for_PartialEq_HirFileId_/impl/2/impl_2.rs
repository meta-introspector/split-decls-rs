use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq < HirFileId > for EditionedFileId { fn eq (& self , & other : & HirFileId) -> bool { other == HirFileId :: from (* self) } }
}