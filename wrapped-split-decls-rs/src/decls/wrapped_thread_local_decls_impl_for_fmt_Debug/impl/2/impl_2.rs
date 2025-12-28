use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T : Send + fmt :: Debug > fmt :: Debug for IterMut < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("IterMut") . field ("raw" , & self . raw) . finish () } }
}