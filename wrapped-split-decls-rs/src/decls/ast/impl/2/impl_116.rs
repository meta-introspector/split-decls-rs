use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for GenBlockKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . modifier () . fmt (f) } }
}