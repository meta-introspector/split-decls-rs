use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Macro > for ItemInNs { fn from (it : Macro) -> Self { Self :: Macros (it) } }
}