use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Hashes strings <= 16 bytes, has unspecified behavior when bytes.len() > 16."] # [inline (always)] fn hash_bytes_short (bytes : & [u8] , accumulator : u64 , seeds : & [u64 ; 6]) -> u64 { let len = bytes . len () ; let mut s0 = accumulator ; let mut s1 = seeds [1] ; if len >= 8 { s0 ^= u64 :: from_ne_bytes (bytes [0 .. 8] . try_into () . unwrap ()) ; s1 ^= u64 :: from_ne_bytes (bytes [len - 8 ..] . try_into () . unwrap ()) ; } else if len >= 4 { s0 ^= u32 :: from_ne_bytes (bytes [0 .. 4] . try_into () . unwrap ()) as u64 ; s1 ^= u32 :: from_ne_bytes (bytes [len - 4 ..] . try_into () . unwrap ()) as u64 ; } else if len > 0 { let lo = bytes [0] ; let mid = bytes [len / 2] ; let hi = bytes [len - 1] ; s0 ^= lo as u64 ; s1 ^= ((hi as u64) << 8) | mid as u64 ; } folded_multiply (s0 , s1) }
}