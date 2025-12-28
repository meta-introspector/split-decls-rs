use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < V : Hash + Eq > From < FxHashSet < V > > for UnordSet < V > { fn from (value : FxHashSet < V >) -> Self { UnordSet { inner : value } } }
}