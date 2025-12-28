use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > Extend < T > for SmallVec < T , N > { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { # [cfg (feature = "specialization")] { spec_traits :: SpecExtend :: < T , _ > :: spec_extend (self , iter . into_iter ()) ; } # [cfg (not (feature = "specialization"))] { self . extend_fallback (iter) ; } } }
}