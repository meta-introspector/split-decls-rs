use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ChunkedBitIter < 'a , T : Idx > { bit_set : & 'a ChunkedBitSet < T > , chunk_index : usize , chunk_iter : ChunkIter < 'a > , }