use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A correctly sized stack allocation for the formatted integer to be written"] # [doc = " into."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mut buffer = itoa::Buffer::new();"] # [doc = " let printed = buffer.format(1234);"] # [doc = " assert_eq!(printed, \"1234\");"] # [doc = " ```"] pub struct Buffer { bytes : [MaybeUninit < u8 > ; i128 :: MAX_STR_LEN] , }