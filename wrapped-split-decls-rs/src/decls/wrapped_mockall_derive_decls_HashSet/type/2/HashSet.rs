use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type HashSet < K > = std :: collections :: HashSet < K , BuildHasherDefault < std :: collections :: hash_map :: DefaultHasher > > ;
}