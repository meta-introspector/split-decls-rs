use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " The maximal size of a dictionary."] pub const DICT_SIZE_MAX : u32 = u32 :: MAX & ! 15_u32 ;
}