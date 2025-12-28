use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MockAttribute { pub fn meta_item_list (self) -> Vec < MockMetaItem > { vec ! [MockMetaItem] } }
}