use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : AsRef < str > > Eq for UniCase < S > { }
}