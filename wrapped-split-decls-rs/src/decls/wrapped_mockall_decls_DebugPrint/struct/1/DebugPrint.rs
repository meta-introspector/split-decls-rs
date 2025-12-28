use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc (hidden)] pub struct DebugPrint < 'a , T : Debug > (pub & 'a T) ;
}