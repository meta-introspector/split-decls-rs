use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A trivial wrapper for [`memmap2::Mmap`] (or `Vec<u8>` on WASM)."] # [cfg (not (any (miri , target_arch = "wasm32")))] pub struct Mmap (memmap2 :: Mmap) ;
}