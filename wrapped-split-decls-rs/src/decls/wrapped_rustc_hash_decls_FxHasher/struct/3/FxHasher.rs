use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A speedy hash algorithm for use within rustc. The hashmap in liballoc"] # [doc = " by default uses SipHash which isn't quite as speedy as we want. In the"] # [doc = " compiler we're not really worried about DOS attempts, so we use a fast"] # [doc = " non-cryptographic hash."] # [doc = ""] # [doc = " The current implementation is a fast polynomial hash with a single"] # [doc = " bit rotation as a finishing step designed by Orson Peters."] # [derive (Clone)] pub struct FxHasher { hash : usize , }
}