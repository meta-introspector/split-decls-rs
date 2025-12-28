use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl std :: fmt :: Display for Fingerprint { fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (formatter , "{:x}-{:x}" , self . 0 , self . 1) } }
}