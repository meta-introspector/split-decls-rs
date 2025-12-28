use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Hash + Eq , V > Extend < (K , V) > for UnordMap < K , V > { # [inline] fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { self . inner . extend (iter) } }
}