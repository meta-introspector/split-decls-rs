use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < C : Extend < T > + UnordCollection , T > ExtendUnord < T > for C { # [inline] fn extend_unord < I : Iterator < Item = T > > (& mut self , items : UnordItems < T , I >) { self . extend (items . 0) } }
}