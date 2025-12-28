use serde::{Deserialize, Serialize};
use std::collections::HashMap;

enum ChunkIter < 'a > { Zeros , Ones (Range < usize >) , Mixed (BitIter < 'a , usize >) , Finished , }