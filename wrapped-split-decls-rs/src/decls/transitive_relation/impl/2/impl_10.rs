use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Deref for TransitiveRelation < T > { type Target = Frozen < TransitiveRelationBuilder < T > > ; fn deref (& self) -> & Self :: Target { & self . builder } }
}