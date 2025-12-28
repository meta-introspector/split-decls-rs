use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (any (miri , target_arch = "wasm32"))] impl Mmap { # [inline] pub unsafe fn map (mut file : File) -> io :: Result < Self > { use std :: io :: Read ; let mut data = Vec :: new () ; file . read_to_end (& mut data) ? ; Ok (Mmap (data)) } }
}