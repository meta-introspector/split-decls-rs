use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Debug for MmapMut { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("MmapMut") . field ("ptr" , & self . as_ptr ()) . field ("len" , & self . len ()) . finish () } }
}