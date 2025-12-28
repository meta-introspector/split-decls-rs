use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < V : Hash + Eq > FromIterator < V > for UnordSet < V > { # [inline] fn from_iter < T : IntoIterator < Item = V > > (iter : T) -> Self { UnordSet { inner : FxHashSet :: from_iter (iter) } } }
}