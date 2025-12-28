use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn chunk_word_index_and_mask < T : Idx > (elem : T) -> (usize , Word) { let chunk_elem = elem . index () % CHUNK_BITS ; word_index_and_mask (chunk_elem) }