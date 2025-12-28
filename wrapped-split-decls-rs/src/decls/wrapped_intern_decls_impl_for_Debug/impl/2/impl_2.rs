use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Debug + Internable + ? Sized > Debug for Interned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self . arc) . fmt (f) } }
}