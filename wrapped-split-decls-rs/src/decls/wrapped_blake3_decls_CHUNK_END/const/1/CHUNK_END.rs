use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const CHUNK_END : u8 = 1 << 1 ;
}