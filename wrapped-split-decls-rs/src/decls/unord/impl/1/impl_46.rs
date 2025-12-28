use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K , V > ! IntoIterator for UnordMap < K , V > { }
}