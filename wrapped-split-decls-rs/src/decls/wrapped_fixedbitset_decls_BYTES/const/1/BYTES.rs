use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [cfg (feature = "serde")] pub (crate) const BYTES : usize = core :: mem :: size_of :: < Block > () ;
}