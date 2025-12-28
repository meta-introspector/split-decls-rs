use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Display for SourceFileHash { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}=" , self . kind) ? ; for byte in self . value [0 .. self . hash_len ()] . into_iter () { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
}