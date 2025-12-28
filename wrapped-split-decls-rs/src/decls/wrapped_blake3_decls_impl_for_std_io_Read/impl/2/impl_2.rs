use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "std")] impl std :: io :: Read for OutputReader { # [inline] fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { self . fill (buf) ; Ok (buf . len ()) } }
}