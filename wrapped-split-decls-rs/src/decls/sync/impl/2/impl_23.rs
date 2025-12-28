use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Eq + Hash , V : Eq , S : BuildHasher > HashMapExt < K , V > for HashMap < K , V , S > { fn insert_same (& mut self , key : K , value : V) { self . entry (key) . and_modify (| old | assert ! (* old == value)) . or_insert (value) ; } }
}