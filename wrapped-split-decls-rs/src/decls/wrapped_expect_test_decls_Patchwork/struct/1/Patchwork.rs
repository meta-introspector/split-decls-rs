use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct Patchwork { text : String , indels : Vec < (Range < usize > , usize) > , }
}