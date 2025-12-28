use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > Extend < & 'a T > for SsoHashSet < T > where T : 'a + Eq + Hash + Copy , { # [inline] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } # [inline] fn extend_one (& mut self , & item : & 'a T) { self . insert (item) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { Extend :: < T > :: extend_reserve (self , additional) } }