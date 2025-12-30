use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: store_in_lru_cache");
pub fn store_in_lru_cache (key : String , value : Vec < u8 >) { if let Ok (mut cache) = LRU_CACHE . lock () { cache . put (key , value) ; } }
}