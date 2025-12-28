use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Implements `serde::Serialize` to write `Event` data to a serializer."] # [derive (Debug)] pub struct SerializeEvent < 'a > (& 'a Event < 'a >) ;
}