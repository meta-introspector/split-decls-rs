use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn broadcast < R : DynSend > (op : impl Fn (usize) -> R + DynSync) -> Vec < R > { if mode :: is_dyn_thread_safe () { let op = FromDyn :: from (op) ; let results = rustc_thread_pool :: broadcast (| context | op . derive (op (context . index ()))) ; results . into_iter () . map (| r | r . into_inner ()) . collect () } else { vec ! [op (0)] } }
}