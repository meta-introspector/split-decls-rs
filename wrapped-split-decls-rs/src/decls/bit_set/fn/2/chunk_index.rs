use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: chunk_index");
# [inline] fn chunk_index < T : Idx > (elem : T) -> usize { elem . index () / CHUNK_BITS }
}