use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T > Extend < T > for IndexVec < I , T > { # [inline] fn extend < J : IntoIterator < Item = T > > (& mut self , iter : J) { self . raw . extend (iter) ; } # [inline] # [cfg (feature = "nightly")] fn extend_one (& mut self , item : T) { self . raw . push (item) ; } # [inline] # [cfg (feature = "nightly")] fn extend_reserve (& mut self , additional : usize) { self . raw . reserve (additional) ; } }
}