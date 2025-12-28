use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > FoldWhile < T > { # [doc = " Return the value in the continue or done."] pub fn into_inner (self) -> T { match self { Self :: Continue (x) | Self :: Done (x) => x , } } # [doc = " Return true if `self` is `Done`, false if it is `Continue`."] pub fn is_done (& self) -> bool { match * self { Self :: Continue (_) => false , Self :: Done (_) => true , } } }
}