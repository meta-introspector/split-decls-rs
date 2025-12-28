use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "std")] impl < R : TryRngCore > std :: io :: Read for RngReader < R > { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize , std :: io :: Error > { self . 0 . try_fill_bytes (buf) . map_err (| err | std :: io :: Error :: other (std :: format ! ("RNG error: {err}"))) ? ; Ok (buf . len ()) } }
}