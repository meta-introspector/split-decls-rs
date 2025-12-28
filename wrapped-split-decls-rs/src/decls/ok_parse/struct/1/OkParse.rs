use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Success variant."] pub struct OkParse < T > (pub T) ;
}