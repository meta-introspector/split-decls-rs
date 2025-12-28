use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn force_stack_overflow (depth : u32) { let mut buffer = [0u8 ; 1024 * 1024] ; # [allow (clippy :: incompatible_msrv)] std :: hint :: black_box (& mut buffer) ; if depth > 0 { force_stack_overflow (depth - 1) ; } }
}