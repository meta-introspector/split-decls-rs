use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("id" , & self . 0) . field ("name" , & self . name ()) . finish () } }
}