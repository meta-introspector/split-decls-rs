use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type ForeignItem = Item < ForeignItemKind > ;
}