use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ExactSizeIterator for IdxRange < T > { }
}