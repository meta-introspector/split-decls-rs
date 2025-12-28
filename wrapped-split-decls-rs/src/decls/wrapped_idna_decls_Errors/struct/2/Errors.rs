use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Type indicating that there were errors during UTS #46 processing."] # [derive (Default , Debug)] # [non_exhaustive] pub struct Errors { }
}