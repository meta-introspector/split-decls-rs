use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const _ : () = assert ! (CHUNK_BITS <= ChunkSize :: MAX as usize) ;