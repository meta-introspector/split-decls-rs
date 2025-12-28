use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: from_le_bytes");
fn from_le_bytes (ty : Type , from : & [u8]) -> Result < u64 > { match ty { Type :: U32 if from . len () == 4 => Ok (u32 :: from_le_bytes (from . try_into () . unwrap ()) . into ()) , Type :: U64 if from . len () == 8 => Ok (u64 :: from_le_bytes (from . try_into () . unwrap ())) , _ => Err (invalid_data ()) , } }
}