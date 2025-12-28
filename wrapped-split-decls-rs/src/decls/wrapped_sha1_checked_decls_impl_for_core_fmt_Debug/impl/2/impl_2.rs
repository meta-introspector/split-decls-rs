use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl core :: fmt :: Debug for Sha1 { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { f . write_str ("Sha1CollisionDetection { .. }") } }
}