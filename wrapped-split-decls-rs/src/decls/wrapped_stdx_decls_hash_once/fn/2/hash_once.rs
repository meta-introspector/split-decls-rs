use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn hash_once < Hasher : std :: hash :: Hasher + Default > (thing : impl std :: hash :: Hash) -> u64 { std :: hash :: BuildHasher :: hash_one (& std :: hash :: BuildHasherDefault :: < Hasher > :: default () , thing) }
}