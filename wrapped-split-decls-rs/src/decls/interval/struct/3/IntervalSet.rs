use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Stores a set of intervals on the indices."] # [doc = ""] # [doc = " The elements in `map` are sorted and non-adjacent, which means"] # [doc = " the second value of the previous element is *greater* than the"] # [doc = " first value of the following element."] # [derive (Debug , Clone)] pub struct IntervalSet < I > { map : SmallVec < [(u32 , u32) ; 2] > , domain : usize , _data : PhantomData < I > , }
}