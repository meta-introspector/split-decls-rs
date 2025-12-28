use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < V : Hash + Eq , I : Iterator < Item = V > > From < UnordItems < V , I > > for UnordSet < V > { fn from (value : UnordItems < V , I >) -> Self { UnordSet { inner : FxHashSet :: from_iter (value . 0) } } }