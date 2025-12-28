use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Send > FusedIterator for IntoIter < T > { }
}