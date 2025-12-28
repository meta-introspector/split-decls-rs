use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone)] struct CacheEntry { time_stamp : usize , line_number : usize , line : Range < BytePos > , file : Arc < SourceFile > , file_index : usize , }
}