use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < ConstParamId > for TypeOrConstParamId { fn from (it : ConstParamId) -> Self { it . 0 } }
}