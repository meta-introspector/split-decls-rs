use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > From < Cow < 'a , str > > for SmolStr { # [inline] fn from (s : Cow < 'a , str >) -> SmolStr { SmolStr :: new (s) } }
}