use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: encode");
# [doc = " Encodes `data` as hex string using lowercase characters."] # [doc = ""] # [doc = " Lowercase characters are used (e.g. `f9b4ca`). The resulting string's"] # [doc = " length is always even, each byte in `data` is always encoded using two hex"] # [doc = " digits. Thus, the resulting string contains exactly twice as many bytes as"] # [doc = " the input data."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(hex::encode(\"Hello world!\"), \"48656c6c6f20776f726c6421\");"] # [doc = " assert_eq!(hex::encode(vec![1, 2, 3, 15, 16]), \"0102030f10\");"] # [doc = " ```"] # [must_use] # [cfg (feature = "alloc")] pub fn encode < T : AsRef < [u8] > > (data : T) -> String { let data = data . as_ref () ; let mut out = vec ! [0 ; data . len () * 2] ; encode_to_slice (data , & mut out) . unwrap () ; String :: from_utf8 (out) . unwrap () }
}