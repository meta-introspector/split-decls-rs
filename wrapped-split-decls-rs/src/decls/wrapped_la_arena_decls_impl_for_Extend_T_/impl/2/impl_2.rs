use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Extend < T > for Arena < T > { fn extend < II : IntoIterator < Item = T > > (& mut self , iter : II) { for t in iter { self . alloc (t) ; } } }
}