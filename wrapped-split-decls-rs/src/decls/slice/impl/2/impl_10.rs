use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , T , R : IntoSliceIdx < I , [T] > > Index < R > for IndexSlice < I , T > { type Output = < R :: Output as SliceIndex < [T] > > :: Output ; # [inline] fn index (& self , index : R) -> & Self :: Output { & self . raw [index . into_slice_idx ()] } }
}