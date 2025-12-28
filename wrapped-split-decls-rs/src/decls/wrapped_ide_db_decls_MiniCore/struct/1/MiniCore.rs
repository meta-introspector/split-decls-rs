use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Copy)] pub struct MiniCore < 'a > (& 'a str) ;
}