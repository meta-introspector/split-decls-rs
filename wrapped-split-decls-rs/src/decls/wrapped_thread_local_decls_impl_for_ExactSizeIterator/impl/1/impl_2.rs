use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Send > ExactSizeIterator for IntoIter < T > { }
}