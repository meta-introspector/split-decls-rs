use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , A : Array < Item = T > > FlatMapInPlace < T > for SmallVec < A > { flat_map_in_place ! (SmallVec where T : Array) ; }
}