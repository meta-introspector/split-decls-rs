use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , 'b > IntoIterator for & 'a StringWrapper < 'b > { type Item = char ; type IntoIter = Chars < 'b > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . chars () } }
}