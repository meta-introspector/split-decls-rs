use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type UnhashSet < V > = HashSet < V , BuildHasherDefault < Unhasher > > ;
}