use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for Svh { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . to_hex ()) } }
}