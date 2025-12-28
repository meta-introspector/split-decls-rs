use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An integer that will always encode to 8 bytes."] pub struct IntEncodedWithFixedSize (pub u64) ;
}