use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ops :: Index < usize > for Slab < T > { type Output = T ; # [track_caller] fn index (& self , key : usize) -> & T { match self . entries . get (key) { Some (Entry :: Occupied (v)) => v , _ => panic ! ("invalid key") , } } }
}