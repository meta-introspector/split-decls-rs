use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Compute Adler-32 hash on `Adler32Hash` type."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `hash` - A Adler-32 hash-able type."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust"] # [doc = " use simd_adler32::adler32;"] # [doc = ""] # [doc = " let hash = adler32(b\"Adler-32\");"] # [doc = " println!(\"{}\", hash); // 800813569"] # [doc = " ```"] pub fn adler32 < H : Adler32Hash > (hash : & H) -> u32 { hash . hash () }
}