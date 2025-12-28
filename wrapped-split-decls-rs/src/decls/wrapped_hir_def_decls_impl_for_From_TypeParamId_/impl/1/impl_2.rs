use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < TypeParamId > for TypeOrConstParamId { fn from (it : TypeParamId) -> Self { it . 0 } }
}