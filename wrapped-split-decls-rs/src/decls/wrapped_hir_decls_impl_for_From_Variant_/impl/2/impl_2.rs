use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < & Variant > for DefWithBodyId { fn from (& v : & Variant) -> Self { DefWithBodyId :: VariantId (v . into ()) } }
}