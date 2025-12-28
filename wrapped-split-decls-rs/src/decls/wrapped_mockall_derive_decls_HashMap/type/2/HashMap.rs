use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type HashMap < K , V > = std :: collections :: HashMap < K , V , BuildHasherDefault < std :: collections :: hash_map :: DefaultHasher > > ;
}