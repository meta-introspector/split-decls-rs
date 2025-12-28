use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < EnumVariantId > for DefWithBodyId { fn from (id : EnumVariantId) -> Self { DefWithBodyId :: VariantId (id) } }
}