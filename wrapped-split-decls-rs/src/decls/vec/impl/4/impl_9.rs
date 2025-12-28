use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc = " `IndexVec` is often used as a map, so it provides some map-like APIs."] impl < I : Idx , T > IndexVec < I , Option < T > > { # [inline] pub fn insert (& mut self , index : I , value : T) -> Option < T > { self . ensure_contains_elem (index , | | None) . replace (value) } # [inline] pub fn get_or_insert_with (& mut self , index : I , value : impl FnOnce () -> T) -> & mut T { self . ensure_contains_elem (index , | | None) . get_or_insert_with (value) } # [inline] pub fn remove (& mut self , index : I) -> Option < T > { self . get_mut (index) ? . take () } # [inline] pub fn contains (& self , index : I) -> bool { self . get (index) . and_then (Option :: as_ref) . is_some () } }
}