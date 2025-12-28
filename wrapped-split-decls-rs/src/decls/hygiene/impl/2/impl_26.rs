use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Debug for ExpnId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}::{{{{expn{}}}}}" , self . krate , self . local_id . as_u32 ()) } }
}