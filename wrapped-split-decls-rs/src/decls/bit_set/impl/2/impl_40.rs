use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : Idx > ChunkedBitIter < 'a , T > { # [inline] fn new (bit_set : & 'a ChunkedBitSet < T >) -> ChunkedBitIter < 'a , T > { ChunkedBitIter { bit_set , chunk_index : 0 , chunk_iter : bit_set . chunk_iter (0) } } }