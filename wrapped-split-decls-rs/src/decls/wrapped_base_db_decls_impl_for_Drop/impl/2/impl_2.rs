use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Drop for DbPanicContext { fn drop (& mut self) { Self :: with_ctx (| ctx | assert ! (ctx . pop () . is_some ())) ; } }
}