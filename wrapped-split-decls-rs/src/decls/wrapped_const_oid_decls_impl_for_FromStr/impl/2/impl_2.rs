use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromStr for ObjectIdentifier { type Err = Error ; fn from_str (string : & str) -> Result < Self > { Self :: new (string) } }
}