use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T > PartialEq for Interned < 'a , T > { # [inline] fn eq (& self , other : & Self) -> bool { ptr :: eq (self . 0 , other . 0) } }
}