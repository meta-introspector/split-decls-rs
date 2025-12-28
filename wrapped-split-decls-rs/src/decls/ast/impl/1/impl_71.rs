use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Box < Pat > > for Pat { fn from (value : Box < Pat >) -> Self { * value } }
}