use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Size of serialized `BlockBuffer` in bytes."] pub type SerializedBufferSize < BS , K > = Sum < BS , < K as sealed :: Sealed > :: Overhead > ;
}