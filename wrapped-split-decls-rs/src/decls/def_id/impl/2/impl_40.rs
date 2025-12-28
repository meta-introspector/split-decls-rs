use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Debug for LocalDefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . to_def_id () . fmt (f) } }
}