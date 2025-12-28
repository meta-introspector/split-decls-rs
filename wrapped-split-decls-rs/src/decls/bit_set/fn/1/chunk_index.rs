use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn chunk_index < T : Idx > (elem : T) -> usize { elem . index () / CHUNK_BITS }