use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Debug for HirId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "HirId({:?}.{:?})" , self . owner , self . local_id) } }
}