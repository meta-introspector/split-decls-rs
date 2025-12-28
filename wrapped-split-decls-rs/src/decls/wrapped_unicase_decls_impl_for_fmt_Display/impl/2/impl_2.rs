use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : fmt :: Display > fmt :: Display for UniCase < S > { # [inline] fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (inner ! (self . 0) , fmt) } }
}