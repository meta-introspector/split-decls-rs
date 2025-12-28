use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Debug for XorShiftRng { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "XorShiftRng {{}}") } }
}