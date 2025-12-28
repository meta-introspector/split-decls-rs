use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Debug for Literal { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , f) } }
}