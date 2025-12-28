use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const CHUNK_BITS : usize = CHUNK_WORDS * WORD_BITS ;
}