use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < V > ! IntoIterator for UnordSet < V > { }
}