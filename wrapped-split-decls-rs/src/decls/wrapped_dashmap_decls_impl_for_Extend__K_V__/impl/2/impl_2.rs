use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Eq + Hash , V , S : BuildHasher + Clone > Extend < (K , V) > for DashMap < K , V , S > { fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , intoiter : I) { for pair in intoiter . into_iter () { self . insert (pair . 0 , pair . 1) ; } } }
}