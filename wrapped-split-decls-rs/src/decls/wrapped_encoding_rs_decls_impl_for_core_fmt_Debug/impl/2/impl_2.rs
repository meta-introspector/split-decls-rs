use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl core :: fmt :: Debug for Encoding { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Encoding {{ {} }}" , self . name) } }
}