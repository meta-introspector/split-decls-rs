use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Bom { # [doc = " Returns the default/empty BOM type, `Bom::Null`."] fn default () -> Self { Bom :: Null } }
}