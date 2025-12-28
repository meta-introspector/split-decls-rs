use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An entry in an `LRUCache`."] # [derive (Debug , Clone)] struct Entry < T > { val : T , # [doc = " Index of the previous entry. If this entry is the head, ignore this field."] prev : u16 , # [doc = " Index of the next entry. If this entry is the tail, ignore this field."] next : u16 , }
}