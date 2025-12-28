use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > FlatMapInPlace < T > for Vec < T > { flat_map_in_place ! (Vec) ; }
}