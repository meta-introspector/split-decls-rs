use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum ChunkIter < 'a > { Zeros , Ones (Range < usize >) , Mixed (BitIter < 'a , usize >) , Finished , }
}