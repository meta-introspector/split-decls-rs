use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq < HirFileId > for EditionedFileId { fn eq (& self , & other : & HirFileId) -> bool { other == HirFileId :: from (* self) } }