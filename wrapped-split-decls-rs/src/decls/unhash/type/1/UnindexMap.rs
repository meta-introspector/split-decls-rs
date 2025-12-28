use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type UnindexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < Unhasher > > ;
}