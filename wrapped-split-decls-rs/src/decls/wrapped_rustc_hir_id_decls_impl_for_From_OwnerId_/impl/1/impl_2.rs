use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < OwnerId > for DefId { fn from (value : OwnerId) -> Self { value . to_def_id () } }
}