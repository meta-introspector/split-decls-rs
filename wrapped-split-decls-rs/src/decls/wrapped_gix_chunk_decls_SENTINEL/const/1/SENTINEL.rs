use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A special value denoting the end of the chunk file table of contents."] pub const SENTINEL : Id = [0u8 ; 4] ;