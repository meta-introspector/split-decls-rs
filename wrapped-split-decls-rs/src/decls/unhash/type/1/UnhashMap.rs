use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type UnhashMap < K , V > = HashMap < K , V , BuildHasherDefault < Unhasher > > ;