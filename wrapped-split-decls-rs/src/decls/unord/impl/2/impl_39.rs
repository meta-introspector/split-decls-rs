use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Extend < T > for UnordBag < T > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . inner . extend (iter) } }
}