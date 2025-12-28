use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < VariantId > for AttrDefId { fn from (vid : VariantId) -> Self { match vid { VariantId :: EnumVariantId (id) => id . into () , VariantId :: StructId (id) => id . into () , VariantId :: UnionId (id) => id . into () , } } }
}