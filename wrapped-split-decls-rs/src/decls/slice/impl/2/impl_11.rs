use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T , R : IntoSliceIdx < I , [T] > > IndexMut < R > for IndexSlice < I , T > { # [inline] fn index_mut (& mut self , index : R) -> & mut Self :: Output { & mut self . raw [index . into_slice_idx ()] } }