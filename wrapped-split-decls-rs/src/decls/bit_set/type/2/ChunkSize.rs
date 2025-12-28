use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " ChunkSize is small to keep `Chunk` small. The static assertion ensures it's"] # [doc = " not too small."] type ChunkSize = u16 ;
}