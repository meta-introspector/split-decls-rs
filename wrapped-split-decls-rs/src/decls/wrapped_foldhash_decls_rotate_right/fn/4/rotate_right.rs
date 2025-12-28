use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: rotate_right");
# [inline (always)] const fn rotate_right (x : u64 , r : u32) -> u64 { # [cfg (any (target_pointer_width = "64" , target_arch = "aarch64" , target_arch = "x86_64" , target_family = "wasm" ,))] { x . rotate_right (r) } # [cfg (not (any (target_pointer_width = "64" , target_arch = "aarch64" , target_arch = "x86_64" , target_family = "wasm" ,)))] { let lo = (x as u32) . rotate_right (r) ; let hi = ((x >> 32) as u32) . rotate_right (r) ; ((hi as u64) << 32) | lo as u64 } }
}