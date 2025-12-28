use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Implements `serde::Serialize` to write `Record` data to a serializer."] # [derive (Debug)] pub struct SerializeRecord < 'a > (& 'a Record < 'a >) ;
}