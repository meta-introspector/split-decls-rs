use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: load");
# [doc = " Load 8 bytes into a u64 word at the given offset."] # [doc = ""] # [doc = " # Safety"] # [doc = " You must ensure that offset + 8 <= bytes.len()."] # [inline (always)] unsafe fn load (bytes : & [u8] , offset : usize) -> u64 { unsafe { bytes . as_ptr () . add (offset) . cast :: < u64 > () . read_unaligned () } }
}