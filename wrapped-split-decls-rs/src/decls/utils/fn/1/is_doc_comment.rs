use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (super) fn is_doc_comment (attr : & Attribute) -> bool { attr . path () . segments . last () . unwrap () . ident == "doc" }