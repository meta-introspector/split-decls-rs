use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: hash");
# [doc = " Computes the CRC32 hash of a byte slice."] # [doc = ""] # [doc = " Check out [`Hasher`] for more advanced use-cases."] pub fn hash (buf : & [u8]) -> u32 { let mut h = Hasher :: new () ; h . update (buf) ; h . finalize () }
}