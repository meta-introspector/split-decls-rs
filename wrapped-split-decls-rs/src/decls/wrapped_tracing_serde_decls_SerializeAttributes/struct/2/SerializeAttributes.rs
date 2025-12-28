use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Implements `serde::Serialize` to write `Attributes` data to a serializer."] # [derive (Debug)] pub struct SerializeAttributes < 'a > (& 'a Attributes < 'a >) ;
}