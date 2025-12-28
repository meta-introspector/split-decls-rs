use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone)] struct Output { input_chaining_value : CVWords , block : [u8 ; 64] , block_len : u8 , counter : u64 , flags : u8 , platform : Platform , }