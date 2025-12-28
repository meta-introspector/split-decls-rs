use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Idx for u32 { # [inline] fn new (idx : usize) -> Self { assert ! (idx <= u32 :: MAX as usize) ; idx as u32 } # [inline] fn index (self) -> usize { self as usize } }
}