use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl std :: fmt :: Debug for InlineAsmOptions { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { bitflags :: parser :: to_writer (self , f) } }
}