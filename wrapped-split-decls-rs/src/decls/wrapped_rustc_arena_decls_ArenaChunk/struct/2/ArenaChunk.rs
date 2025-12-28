use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct ArenaChunk < T = u8 > { # [doc = " The raw storage for the arena chunk."] storage : NonNull < [MaybeUninit < T >] > , # [doc = " The number of valid entries in the chunk."] entries : usize , }