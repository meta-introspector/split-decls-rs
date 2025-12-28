use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " This no-op hasher expects only a single `write_u64` call. It's intended for"] # [doc = " map keys that already have hash-like quality, like `Fingerprint`."] # [derive (Default)] pub struct Unhasher { value : u64 , }
}