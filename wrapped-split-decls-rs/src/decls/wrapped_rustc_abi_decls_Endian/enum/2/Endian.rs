use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Endianness of the target, which must match cfg(target-endian)."] # [derive (Copy , Clone , PartialEq , Eq)] pub enum Endian { Little , Big , }
}