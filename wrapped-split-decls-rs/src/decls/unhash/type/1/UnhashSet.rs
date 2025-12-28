use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type UnhashSet < V > = HashSet < V , BuildHasherDefault < Unhasher > > ;