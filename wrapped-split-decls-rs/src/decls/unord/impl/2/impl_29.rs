use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Eq + Hash , V > Default for UnordMap < K , V > { # [inline] fn default () -> Self { Self { inner : FxHashMap :: default () } } }
}