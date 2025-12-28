use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " The size of the buffer in `FileEncoder`."] const BUF_SIZE : usize = 64 * 1024 ;
}