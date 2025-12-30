use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: str_assume_utf8");
# [inline] unsafe fn str_assume_utf8 (string : & OsStr) -> & str { # [cfg (os_str_bytes)] { unsafe { std :: str :: from_utf8_unchecked (# [allow (clippy :: incompatible_msrv)] string . as_encoded_bytes () ,) } } # [cfg (not (os_str_bytes))] { match string . to_str () { Some (val) => val , None => std :: hint :: unreachable_unchecked () , } } }
}