use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K , V , S > Default for DashMap < K , V , S > where K : Eq + Hash , S : Default + BuildHasher + Clone , { fn default () -> Self { Self :: with_hasher (Default :: default ()) } }
}