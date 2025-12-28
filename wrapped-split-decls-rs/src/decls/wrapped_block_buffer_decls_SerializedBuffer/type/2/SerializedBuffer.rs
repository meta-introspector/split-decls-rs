use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " `BlockBuffer` serialized as a byte array."] pub type SerializedBuffer < BS , K > = Array < u8 , SerializedBufferSize < BS , K > > ;
}