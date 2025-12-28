use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Eq + Hash > Extend < T > for SsoHashSet < T > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { for val in iter . into_iter () { self . insert (val) ; } } # [inline] fn extend_one (& mut self , item : T) { self . insert (item) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . map . extend_reserve (additional) } }
}