use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [allow (deprecated)] impl fmt :: Debug for Configuration { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . builder . fmt (f) } }
}