use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K , V > Default for IndexMap < K , V > { fn default () -> Self { Self { index_map : FxIndexMap :: default () , } } }
}