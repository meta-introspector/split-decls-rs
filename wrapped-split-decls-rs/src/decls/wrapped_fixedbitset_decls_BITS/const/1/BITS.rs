use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
pub (crate) const BITS : usize = core :: mem :: size_of :: < Block > () * 8 ;
}