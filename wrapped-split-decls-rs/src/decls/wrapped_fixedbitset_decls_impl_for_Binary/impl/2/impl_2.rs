use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Binary for FixedBitSet { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { if f . alternate () { f . write_str ("0b") ? ; } for i in 0 .. self . length { if self [i] { f . write_char ('1') ? ; } else { f . write_char ('0') ? ; } } Ok (()) } }
}