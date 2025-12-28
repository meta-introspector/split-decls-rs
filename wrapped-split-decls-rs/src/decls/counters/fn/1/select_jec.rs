use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn select_jec (word : usize) -> usize { word >> JEC_SHIFT }
}