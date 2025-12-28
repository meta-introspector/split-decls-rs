use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub (crate) type HashMap < K , V > = hash_table :: HashTable < (K , V) > ;
}